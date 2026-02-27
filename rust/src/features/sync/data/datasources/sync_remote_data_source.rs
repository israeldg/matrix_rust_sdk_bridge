use std::sync::Arc;

use crate::core::error::failure::CustomFailure;

use futures_util::stream::{BoxStream, StreamExt};
use matrix_sdk::config::SyncSettings;
use matrix_sdk::Room;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
//use matrix_sdk::ruma::api::client::filter::FilterDefinition;

use matrix_sdk::{
    ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent},
    Client,
};
use matrix_sdk_ui::sync_service::SyncService;
/// Represents an abstraction over the remote Matrix API data source.
/// This version does not use `async_trait` — instead, methods return
/// synchronous `Result`s or expose `Receiver` streams.
pub trait SyncRemoteDataSource {
    // Sync management
    //async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure>;
    async fn sync(&self) -> Result<BoxStream<'static, String>, CustomFailure>;
    //async fn sync_events(&self) -> Result<(), CustomFailure>;
}

pub struct SyncRemoteDataSourceImpl {
    matrix_client: Client,
    sync_service: Option<Arc<SyncService>>,
}

impl SyncRemoteDataSourceImpl {
    pub fn new(matrix_client: Client) -> Self {
        Self {
            matrix_client,
            sync_service: None,
        }
    }
}

impl SyncRemoteDataSource for SyncRemoteDataSourceImpl {
    /// Performs a single sync operation with the Matrix server.
    ///
    /// # Arguments
    /// * `initial_sync_token` - Optional sync token to resume from a previous sync
    ///
    /// # Returns
    /// The next_batch token to use for subsequent syncs
    // async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure> {
    //     // Enable lazy-loading to speed up initial sync for accounts with many rooms
    //     // See: https://spec.matrix.org/v1.6/client-server-api/#lazy-loading-room-members
    //     let filter = FilterDefinition::with_lazy_loading();
    //     let mut sync_settings = SyncSettings::default().filter(filter.into());

    //     // Resume from previous sync position if token provided
    //     if let Some(sync_token) = initial_sync_token {
    //         sync_settings = sync_settings.token(sync_token);
    //     }

    //     // Retry loop for handling transient network errors
    //     loop {
    //         match self.matrix_client.sync_once(sync_settings.clone()).await {
    //             Ok(response) => {
    //                 // Return the next_batch token for the next sync
    //                 return Ok(response.next_batch);
    //             }
    //             Err(error) => {
    //                 println!("RUST: An error occurred during initial sync: {error}");
    //                 println!("RUST: Trying again…");
    //             }
    //         }
    //     }

    //     // // 1. Declare a variable to store the next_batch value
    //     // let next_batch_token: String;

    //     // loop {
    //     //     match self.matrix_client.sync_once(sync_settings.clone()).await {
    //     //         Ok(response) => {
    //     //             // This is the last time we need to provide this token, the sync method after
    //     //             // will handle it on its own.
    //     //             next_batch_token = response.next_batch;
    //     //             break;
    //     //         }
    //     //         Err(error) => {
    //     //             println!("An error occurred during initial sync: {error}");
    //     //             println!("Trying again…");
    //     //         }
    //     //     }
    //     // }

    //     // Ok(next_batch_token)
    // }

    /// Starts a continuous sync stream that sends message updates to Flutter.
    /// Uses flutter_rust_bridge::spawn for cross-platform compatibility.
    ///
    ///
    /// # Important
    /// Stream automatically stops when Flutter disposes the stream or an error occurs.
    async fn sync(&self) -> Result<BoxStream<'static, String>, CustomFailure> {
        let client: Client = self.matrix_client.clone();

        // 1. Create a channel to act as the bridge
        // Buffer of 100 messages to handle bursts
        let (tx, rx) = mpsc::channel::<String>(100);

        // 2. Add the event handler
        // The handler stays inside the SDK/Client logic
        client.add_event_handler({
            let tx = tx.clone();
            move |event: OriginalSyncRoomMessageEvent, _room: Room| {
                let tx = tx.clone();
                async move {
                    if let MessageType::Text(text_content) = event.content.msgtype {
                        // Send the message into our internal bridge
                        let _ = tx.send(text_content.body).await;
                    }
                }
            }
        });

        // 3. Trigger the SDK sync in the background
        // We spawn this so the function can return the stream immediately
        flutter_rust_bridge::spawn(async move {
            if let Err(e) = client.sync(SyncSettings::default()).await {
                eprintln!("Sync error: {:?}", e);
                // Optionally send an error message through the channel or handle it
            }
        });

        // 4. Convert the MPSC receiver into a Stream and box it
        let stream = ReceiverStream::new(rx);

        Ok(stream.boxed())
    }

    // async fn sync_events(&self) -> Result<(), CustomFailure> {
    //     let sync_service = Arc::new(
    //         SyncService::builder(self.matrix_client.clone())
    //             .with_share_pos(true)
    //             .build()
    //             .await
    //             .map_err(|err| {
    //                 eprintln!("RUST: Failed to build SyncService: {:?}", err); // Side effect
    //                 CustomFailure::Unknown(format!("Unable to build SyncService: {}", err))
    //                 // Error conversion
    //             })?,
    //     );

    //     let mut state_subscriber = sync_service.state();

    //     flutter_rust_bridge::spawn(async move {
    //         while let Some(state) = state_subscriber.next().await {
    //             let state_str = match state {
    //                 State::Idle => "idle",
    //                 State::Running => "running",
    //                 State::Offline => "offline",
    //                 State::Terminated => "terminated",
    //                 State::Error(_error) => "error",
    //             };

    //             if sink.add(state_str.to_string()).is_err() {
    //                 break; // Stream closed
    //             }

    //             // Stop if terminated
    //             if state_str == "terminated" {
    //                 break;
    //             }
    //         }
    //     });
    //     // This will sync (with encryption) until an error happens or the program is
    //     // stopped.
    //     sync_service.start().await;

    //     Ok(())
    // }
}
