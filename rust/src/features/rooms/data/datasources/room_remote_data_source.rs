use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use chrono::{DateTime, Utc};

use futures_util::{pin_mut, StreamExt as _};
use imbl::Vector;

use matrix_sdk_ui::room_list_service::RoomListItem;
use matrix_sdk_ui::sync_service::SyncService;
use matrix_sdk_ui::timeline::RoomExt;

use serde_json::Value;

use crate::features::rooms::domain::entities::room::RoomEntity;
use crate::frb_generated::StreamSink;
use crate::{
    core::error::failure::CustomFailure, features::rooms::data::models::room_model::RoomModel,
};
use matrix_sdk::Client;

use futures_util::future::join_all;

pub type Rooms = Arc<Mutex<Vector<RoomListItem>>>;

/// Represents an abstraction over the remote Matrix API data source.
/// This version does not use `async_trait` — instead, methods return
/// synchronous `Result`s or expose `Receiver` streams.
pub trait RoomRemoteDataSource {
    // Room management
    async fn get_rooms_by_space(
        &self,
        space_id: String,
        sink: StreamSink<Vec<RoomEntity>>,
    ) -> Result<(), CustomFailure>;
    // Spaces
    async fn get_spaces(&self) -> Result<Vec<RoomModel>, CustomFailure>;
}

pub struct RoomRemoteDataSourceImpl {
    matrix_client: Client,
    sync_service: Arc<SyncService>,
}

impl RoomRemoteDataSourceImpl {
    pub fn new(matrix_client: Client, sync_service: Arc<SyncService>) -> Self {
        Self {
            matrix_client,
            sync_service,
        }
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

    async fn get_rooms_by_space(
        &self,
        space_id: String,
        sink: StreamSink<Vec<RoomEntity>>,
    ) -> Result<(), CustomFailure> {
        //getting the room_list_service ready
        let room_list_service = self.sync_service.room_list_service();
        let all_rooms = match room_list_service.all_rooms().await {
            Ok(all_rooms) => all_rooms,
            Err(error) => {
                println!("RUST: Error logging in: {error}");
                println!("RUST: Please try again\n");
                return Err(CustomFailure::Unknown("Unable to load rooms".to_string()));
            }
        };

        flutter_rust_bridge::spawn(async move {
            let sink: StreamSink<Vec<RoomEntity>> = sink.clone();

            let task_id = uuid::Uuid::new_v4(); // Or use a simple counter

            println!("RUST: Task {} started", task_id);

            let (stream, entries_controller) = all_rooms.entries_with_dynamic_adapters(50_000);

            //setting the filters to the entries_controller, can apply like search, or roomtype etc..
            //it will return the result plus the stream will continually send changes over that filter
            entries_controller.set_filter(Box::new(
                matrix_sdk_ui::room_list_service::filters::new_filter_non_left(),
            ));
            //experiment search by room name
            //entries_controller.set_filter(Box::new(matrix_sdk_ui::room_list_service::filters::new_filter_fuzzy_match_room_name("pattern")));

            pin_mut!(stream);

            let rooms = Rooms::default();

            while let Some(diffs) = stream.next().await {
                let all_rooms_items = {
                    // Apply the diffs to the list of room entries.
                    let mut rooms = match rooms.lock() {
                        Ok(result) => {
                            // Return the next_batch token for the next sync
                            result
                        }
                        Err(_) => {
                            println!("RUST: Task {} - Lock error, breaking", task_id);
                            break;
                        }
                    };

                    for diff in diffs {
                        diff.apply(&mut rooms);
                    }

                    // Collect rooms early to release the room entries list lock.
                    (*rooms).clone()
                };
                // -----------------------------------------------------
                // ⭐ Convert Vector<matrix_sdk::Room> → Vec<RoomEntity>
                // -----------------------------------------------------
                let room_entities: Vec<RoomEntity> =
                    join_all(all_rooms_items.iter().map(|room| async move {
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
                    }))
                    .await;
                println!(
                    "RUST: Task {} - Sending {} rooms",
                    task_id,
                    room_entities.len()
                );
                // -----------------------------------------------------
                // ⭐ Send Vec<RoomEntity> to Flutter
                // -----------------------------------------------------
                if sink.add(room_entities).is_err() {
                    println!("RUST: Task {} - Sink closed, task ending", task_id);
                    break;
                }
            }
            println!(
                "RUST: Task {} - COMPLETED (stream ended or broken)",
                task_id
            );
        });

        Ok(())
    }
}

//helpers
fn system_time_to_datetime_utc(st: SystemTime) -> DateTime<Utc> {
    DateTime::<Utc>::from(st)
}
