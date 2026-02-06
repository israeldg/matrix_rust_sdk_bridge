use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures_util::{pin_mut, StreamExt as _};
use imbl::Vector;
use matrix_sdk::locks::Mutex;
use matrix_sdk_ui::eyeball_im::VectorDiff;

use crate::core::error::failure::CustomFailure;
use crate::features::timeline::data::models::timeline_handle::TimelineHandle;
use crate::features::timeline::domain::entities::event::EventEntity;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;
use crate::frb_generated::StreamSink;

use matrix_sdk::ruma::RoomId;
use matrix_sdk::Client;

use matrix_sdk_ui::sync_service::SyncService;
use matrix_sdk_ui::timeline::{
    MsgLikeContent, MsgLikeKind, RoomExt, TimelineFocus, TimelineItem, TimelineItemContent,
    TimelineItemKind,
};

pub trait TimelineRemoteDataSource {
    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
        sink: StreamSink<Vec<EventDeltaEntity>>,
    ) -> Result<(), CustomFailure>;

    async fn paginate_backwards(&self, room_id: String, limit: u16) -> Result<(), CustomFailure>;
}

pub struct TimelineRemoteDataSourceImpl {
    matrix_client: Client,
    sync_service: Arc<SyncService>,
    timelines: Arc<Mutex<HashMap<String, TimelineHandle>>>,
}

impl TimelineRemoteDataSourceImpl {
    pub fn new(matrix_client: Client, sync_service: Arc<SyncService>) -> Self {
        Self {
            matrix_client,
            sync_service,
            timelines: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // ─────────────────────────────────────────────
    // Conversion helpers
    // ─────────────────────────────────────────────

    fn convert_item(item: &matrix_sdk_ui::timeline::TimelineItem) -> EventEntity {
        use matrix_sdk_ui::timeline::{
            MsgLikeContent, MsgLikeKind, TimelineItemContent, TimelineItemKind,
        };

        match item.kind() {
            TimelineItemKind::Event(ev) => {
                let sender_id = ev.sender().to_string();

                let mut entity = EventEntity {
                    sender_id,
                    ..EventEntity::default()
                };

                match ev.content() {
                    TimelineItemContent::MsgLike(MsgLikeContent {
                        kind: MsgLikeKind::Message(message),
                        ..
                    }) => {
                        entity.content = message.body().to_owned();
                        entity.message_type = "text".into();
                        entity.event_type = "m.room.message".into();
                    }

                    _ => {
                        // 👇 everything else is just test data
                        entity.content = "TEST / UNSUPPORTED EVENT".into();
                        entity.message_type = "unsupported".into();
                        entity.event_type = "test".into();
                    }
                }

                entity
            }

            TimelineItemKind::Virtual(_) => {
                let mut e = EventEntity::default();
                e.content = "TEST / VIRTUAL ITEM".into();
                e
            }
        }
    }

    fn convert_vec(vec: &Vector<Arc<TimelineItem>>) -> Vec<EventEntity> {
        vec.iter()
            .map(|item| Self::convert_item(item.as_ref()))
            .collect()
    }

    /// Stateless diff conversion only
    fn convert_delta(diff: VectorDiff<Arc<TimelineItem>>) -> Vec<EventDeltaEntity> {
        match diff {
            VectorDiff::PushFront { value } => vec![EventDeltaEntity::PushFront {
                value: Self::convert_item(&value),
            }],

            VectorDiff::PushBack { value } => vec![EventDeltaEntity::PushBack {
                value: Self::convert_item(&value),
            }],

            VectorDiff::Insert { index, value } => vec![EventDeltaEntity::Insert {
                index: index as u32,
                value: Self::convert_item(&value),
            }],

            VectorDiff::Set { index, value } => vec![EventDeltaEntity::Update {
                index: index as u32,
                value: Self::convert_item(&value),
            }],

            VectorDiff::Remove { index } => vec![EventDeltaEntity::Remove {
                index: index as u32,
            }],

            VectorDiff::Append { values } => values
                .into_iter()
                .map(|v| EventDeltaEntity::PushBack {
                    value: Self::convert_item(&v),
                })
                .collect(),

            _ => vec![], // handled elsewhere
        }
    }
}

impl TimelineRemoteDataSource for TimelineRemoteDataSourceImpl {
    async fn paginate_backwards(&self, room_id: String, limit: u16) -> Result<(), CustomFailure> {
        let map = self.timelines.lock();
        let handle = map
            .get(&room_id)
            .ok_or(CustomFailure::NotFound("Timeline not initialized".into()))?;

        handle
            .timeline
            .paginate_backwards(limit.into())
            .await
            .map_err(|_| CustomFailure::InvalidInput("Pagination failed".into()))?;

        Ok(())
    }

    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
        sink: StreamSink<Vec<EventDeltaEntity>>,
    ) -> Result<(), CustomFailure> {
        let room_id_parsed = RoomId::parse(&room_id)
            .map_err(|_| CustomFailure::InvalidInput("Invalid room id".into()))?;

        let room = self
            .matrix_client
            .get_room(&room_id_parsed)
            .ok_or(CustomFailure::NotFound("Room not found".into()))?;

        let timeline = Arc::new(
            room.timeline_builder()
                .with_focus(TimelineFocus::Live {
                    hide_threaded_events: true,
                })
                .build()
                .await
                .map_err(|_| CustomFailure::InvalidInput("Failed to build timeline".into()))?,
        );

        let (initial_items, stream) = timeline.subscribe().await;

        let rust_state: Arc<Mutex<Vector<Arc<TimelineItem>>>> = Arc::new(Mutex::new(initial_items));

        let initial_events: Vec<EventEntity> = Self::convert_vec(&rust_state.lock());

        // 🔥 Emit initial snapshot as a Reset delta
        sink.add(vec![EventDeltaEntity::Reset {
            items: initial_events,
        }])
        .ok();

        let state = rust_state.clone();
        let task = flutter_rust_bridge::spawn(async move {
            pin_mut!(stream);

            while let Some(diffs) = stream.next().await {
                let mut state = state.lock();
                let mut deltas = Vec::new();

                for diff in diffs {
                    // 1️⃣ Inspect diff WITHOUT moving it
                    match &diff {
                        VectorDiff::PopFront => {
                            if !state.is_empty() {
                                deltas.push(EventDeltaEntity::Remove { index: 0 });
                            }
                        }

                        VectorDiff::PopBack => {
                            if !state.is_empty() {
                                deltas.push(EventDeltaEntity::Remove {
                                    index: (state.len() - 1) as u32,
                                });
                            }
                        }

                        VectorDiff::Truncate { length } => {
                            for i in (*length..state.len()).rev() {
                                deltas.push(EventDeltaEntity::Remove { index: i as u32 });
                            }
                        }

                        VectorDiff::Clear => {
                            deltas.push(EventDeltaEntity::Reset { items: vec![] });
                        }

                        VectorDiff::Reset { values } => {
                            deltas.push(EventDeltaEntity::Reset {
                                items: values
                                    .iter()
                                    .map(|v| Self::convert_item(v.as_ref()))
                                    .collect(),
                            });
                        }

                        other => {
                            deltas.extend(Self::convert_delta(other.clone()));
                            // ⬆️ safe because convert_delta consumes
                        }
                    }

                    // 2️⃣ Apply AFTER inspection (consumes diff)
                    diff.apply(&mut state);
                }

                sink.add(deltas).ok();
            }
        });

        self.timelines
            .lock()
            .insert(room_id, TimelineHandle { timeline, task });

        Ok(())
    }
}
