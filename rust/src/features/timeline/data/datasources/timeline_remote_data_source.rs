use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{pin_mut, StreamExt as _};
use imbl::Vector;
use matrix_sdk_ui::eyeball_im::VectorDiff;
use tokio::sync::Mutex;

use crate::core::error::failure::CustomFailure;
use crate::features::timeline::data::models::timeline_handle::TimelineHandle;
use crate::features::timeline::domain::entities::event::EventEntity;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;

use matrix_sdk::ruma::RoomId;
use matrix_sdk::Client;

use matrix_sdk_ui::sync_service::SyncService;
use matrix_sdk_ui::timeline::{
    MsgLikeContent, MsgLikeKind, RoomExt, TimelineFocus, TimelineItem, TimelineItemContent,
    TimelineItemKind,
};

use futures_util::stream::BoxStream;

pub trait TimelineRemoteDataSource {
    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
    ) -> Result<BoxStream<'static, Vec<EventDeltaEntity>>, CustomFailure>;

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
        let map = self.timelines.lock().await; // Note: using matrix_sdk Mutex lock
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
    ) -> Result<BoxStream<'static, Vec<EventDeltaEntity>>, CustomFailure> {
        // 1. Create clones *before* the stream block
        let timelines_ptr = self.timelines.clone();
        let room_id_for_cleanup = room_id.clone();

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

        // Subscribe to the SDK timeline
        let (initial_items, stream) = timeline.subscribe().await;

        // Cache the timeline handle for pagination (we don't store the JoinHandle/Task anymore)
        self.timelines.lock().await.insert(
            room_id.clone(),
            TimelineHandle {
                timeline: timeline.clone(),
            },
        );

        let s = async_stream::stream! {
            let rust_state: Arc<Mutex<Vector<Arc<TimelineItem>>>> = Arc::new(Mutex::new(initial_items));

            // 1. Yield Initial Snapshot
            let initial_events: Vec<EventEntity> = {
                let state = rust_state.lock().await;
                Self::convert_vec(&state)
            };
            yield vec![EventDeltaEntity::Reset { items: initial_events }];

            pin_mut!(stream);

            // 2. Continuous Diff Processing
            while let Some(diffs) = stream.next().await {
                let mut state = rust_state.lock().await;
                let mut deltas = Vec::new();

                for diff in diffs {
                    // Inspect and build delta entities
                    match &diff {
                        VectorDiff::PopFront => {
                            if !state.is_empty() { deltas.push(EventDeltaEntity::Remove { index: 0 }); }
                        }
                        VectorDiff::PopBack => {
                            if !state.is_empty() {
                                deltas.push(EventDeltaEntity::Remove { index: (state.len() - 1) as u32 });
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
                                items: values.iter().map(|v| Self::convert_item(v.as_ref())).collect(),
                            });
                        }
                        other => {
                            deltas.extend(Self::convert_delta(other.clone()));
                        }
                    }
                    // Apply the diff to our local state copy
                    diff.apply(&mut state);
                }

                yield deltas;
            }

            // --- THE CLEANUP ---
            // Accessing the cloned variables we defined above
            let mut lock = timelines_ptr.lock().await;
            lock.remove(&room_id_for_cleanup);
        };

        Ok(Box::pin(s))
    }
}
