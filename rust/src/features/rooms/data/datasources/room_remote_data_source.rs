use crate::core::error::failure::CustomFailure;
use crate::features::rooms::data::models::room_model::RoomModel;
use crate::features::rooms::domain::entities::room::RoomEntity;
use chrono::{DateTime, Utc};
use futures_util::{future::join_all, pin_mut, stream::BoxStream, StreamExt};
use imbl::Vector;
use matrix_sdk::Client;
use matrix_sdk_ui::room_list_service::{RoomList, RoomListItem};
use matrix_sdk_ui::sync_service::SyncService;

use matrix_sdk_ui::timeline::RoomExt;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub type Rooms = Arc<Mutex<Vector<RoomListItem>>>;

pub trait RoomRemoteDataSource {
    async fn get_spaces(&self) -> Result<Vec<RoomModel>, CustomFailure>;

    // ⭐ Clean Architecture fix: Return a BoxStream of RoomEntity vectors
    async fn get_rooms_by_space_stream(
        &self,
        _space_id: String,
    ) -> Result<BoxStream<'static, Vec<RoomEntity>>, CustomFailure>;
}

pub struct RoomRemoteDataSourceImpl {
    matrix_client: Client,
    sync_service: Arc<SyncService>,
    all_rooms: Arc<RoomList>,
}

impl RoomRemoteDataSourceImpl {
    pub async fn new(
        matrix_client: Client,
        sync_service: Arc<SyncService>,
    ) -> Result<Self, CustomFailure> {
        let room_list_service = sync_service.room_list_service();
        let all_rooms = room_list_service
            .all_rooms()
            .await
            .map_err(|_| CustomFailure::Unknown("Unable to load rooms".into()))?;

        Ok(Self {
            matrix_client,
            sync_service,
            all_rooms: Arc::new(all_rooms),
        })
    }
}

impl RoomRemoteDataSource for RoomRemoteDataSourceImpl {
    async fn get_spaces(&self) -> Result<Vec<RoomModel>, CustomFailure> {
        let json_str = r#"[
                   {
                       "room_id": "!spaceA:matrix.org",
                       "display_name": "Engineering Space",
                       "avatar_url": "https://picsum.photos/220",
                       "last_event_text": "New sprint planning",
                       "last_event_received_time": "2025-10-24T11:00:00Z",
                       "is_direct_chat": false,
                       "is_encrypted": false,
                       "last_event": "m.room.topic",
                       "participant_count": 25
                   },
                   {
                       "room_id": "!spaceB:matrix.org",
                       "display_name": "Marketing Space",
                       "avatar_url": "https://picsum.photos/230",
                       "last_event_text": "Ad campaign kickoff",
                       "last_event_received_time": "2025-10-24T12:30:00Z",
                       "is_direct_chat": false,
                       "is_encrypted": false,
                       "last_event": "m.room.message",
                       "participant_count": 18
                   }
               ]"#;

        let json_val: Value = serde_json::from_str(json_str)
            .map_err(|e| CustomFailure::InvalidInput(format!("Invalid JSON: {}", e)))?;

        let arr = json_val
            .as_array()
            .ok_or_else(|| CustomFailure::InvalidInput("Expected JSON array".into()))?;

        let mut spaces = Vec::new();
        for v in arr {
            match RoomModel::from_json(v) {
                Ok(model) => spaces.push(model),
                Err(e) => return Err(CustomFailure::InvalidInput(format!("Invalid room: {}", e))),
            }
        }

        Ok(spaces)
    }

    async fn get_rooms_by_space_stream(
        &self,
        _space_id: String,
    ) -> Result<BoxStream<'static, Vec<RoomEntity>>, CustomFailure> {
        let room_list_service = self.sync_service.room_list_service();

        let all_rooms = room_list_service
            .all_rooms()
            .await
            .map_err(|e| CustomFailure::Unknown(format!("Unable to load rooms: {}", e)))?;

        // We use the stream! macro to create a generator-like stream
        let s = async_stream::stream! {
            let (stream, entries_controller) = all_rooms.entries_with_dynamic_adapters(50_000);

            entries_controller.set_filter(Box::new(
                matrix_sdk_ui::room_list_service::filters::new_filter_non_left(),
            ));

            pin_mut!(stream);
            let rooms_state = Rooms::default();

            while let Some(diffs) = stream.next().await {
                let all_rooms_items = {
                    let mut rooms_lock = match rooms_state.lock() {
                        Ok(res) => res,
                        Err(_) => break,
                    };

                    for diff in diffs {
                        diff.apply(&mut rooms_lock);
                    }
                    (*rooms_lock).clone()
                };

                // Perform the async conversion for each room
                let room_entities: Vec<RoomEntity> = join_all(all_rooms_items.iter().map(|room| async move {
                    let room_id = room.room_id().to_string();

                        let display_name = room
                            .cached_display_name()
                            .map(|x| x.to_string())
                            .unwrap_or("Unnamed room".to_string());

                        let is_direct_chat = room
                            .is_direct()
                            .await
                            .map_err(|err| {
                                CustomFailure::DatabaseError(
                                    "couldn't figure whether a room is a DM or not".to_string(),
                                )
                            })
                            .unwrap_or(false);

                        let is_encrypted = room.encryption_state().is_encrypted();
                        let avatar_url = room.avatar_url().map(|m| m.to_string());

                        let last_event_opt = room.latest_event_item().await;

                        let last_event_text = last_event_opt
                            .as_ref()
                            .and_then(|ev| ev.content().as_message())
                            .map(|msg| msg.body().to_string());

                        let last_event_received_time = last_event_opt
                            .as_ref()
                            .and_then(|ev| ev.timestamp().to_system_time())
                            .map(system_time_to_datetime_utc);

                        let last_event = last_event_opt.as_ref().map(|ev| format!("{:?}", ev));

                        let participant_count = Some(room.joined_members_count() as i32);

                        RoomEntity::new(
                            room_id,
                            display_name,
                            is_direct_chat,
                            is_encrypted,
                            avatar_url,
                            last_event_text,
                            last_event_received_time,
                            last_event,
                            participant_count,
                        )
                })).await;

                // ⭐ This "yields" the data to whoever is listening to the stream
                yield room_entities;
            }
        };

        Ok(Box::pin(s))
    }
}

fn system_time_to_datetime_utc(st: SystemTime) -> DateTime<Utc> {
    DateTime::<Utc>::from(st)
}
