use std::sync::Arc;

use matrix_sdk::Client;
use matrix_sdk_ui::sync_service::SyncService;

use crate::{
    core::{
        common::matrix_client_management::matrix_client_context::MatrixClientContext,
        error::failure::CustomFailure,
    },
    features::{
        auth::{
            data::{
                datasources::auth_remote_data_source::AuthRemoteDataSourceImpl,
                repositories::auth_repository_impl::AuthRepositoryImpl,
            },
            usecases::{
                login_matrix_with_password::LoginMatrixWithPassword,
                restore_matrix_session::RestoreMatrixSession,
            },
        },
        matrix_client_registry::domain::entities::registry_session::ClientSessionEntity,
        rooms::{
            data::{
                datasources::room_remote_data_source::RoomRemoteDataSourceImpl,
                repositories::room_repository_impl::RoomRepositoryImpl,
            },
            usecases::{get_rooms::GetRooms, send_message_to_room::SendMessageToRoom},
        },
        sync::{
            data::{
                datasources::sync_remote_data_source::SyncRemoteDataSourceImpl,
                repositories::sync_repository_impl::SyncRepositoryImpl,
            },
            usecases::sync_events::SyncEvents,
        },
        timeline::{
            data::{
                datasources::timeline_remote_data_source::TimelineRemoteDataSourceImpl,
                repositories::timeline_repository_impl::TimelineRepositoryImpl,
            },
            usecases::fetch_room_events_by_room_id::{self, FetchRoomEventsByRoomId},
        },
    },
};

pub struct MatrixClientContextFactory;

impl MatrixClientContextFactory {
    pub async fn create(
        &self,
        client: Client,
        sync_service: Arc<SyncService>,
    ) -> Result<MatrixClientContext, CustomFailure> {
        let auth_remote = AuthRemoteDataSourceImpl::new(client.clone());
        let auth_repo = Arc::new(AuthRepositoryImpl::new(auth_remote));
        // let login_matrix = Arc::new(LoginMatrix::new(auth_repo.clone()));
        let login_matrix_with_password = Arc::new(LoginMatrixWithPassword::new(auth_repo.clone()));
        let restore_matrix_session = Arc::new(RestoreMatrixSession::new(auth_repo.clone()));

        let sync_remote = SyncRemoteDataSourceImpl::new(client.clone());
        let sync_repo = Arc::new(SyncRepositoryImpl::new(sync_remote));
        let sync_events = Arc::new(SyncEvents::new(sync_repo.clone()));

        let room_remote =
            RoomRemoteDataSourceImpl::new(client.clone(), sync_service.clone()).await?;
        let room_repo = Arc::new(RoomRepositoryImpl::new(room_remote));
        let get_rooms = Arc::new(GetRooms::new(room_repo.clone()));
        let send_message_to_room = Arc::new(SendMessageToRoom::new(room_repo.clone()));
        // This will sync (with encryption) until an error happens or the program is
        // stopped.
        let timeline_remote =
            TimelineRemoteDataSourceImpl::new(client.clone(), sync_service.clone());
        let timeline_repo = Arc::new(TimelineRepositoryImpl::new(timeline_remote));
        let fetch_room_events_by_room_id =
            Arc::new(FetchRoomEventsByRoomId::new(timeline_repo.clone()));

        Ok(MatrixClientContext {
            client,
            sync_service,
            login_matrix_with_password,
            restore_matrix_session,
            sync_events,
            get_rooms,
            fetch_room_events_by_room_id,
            send_message_to_room,
        })
    }

    pub async fn build_client(
        &self,
        client_session: &ClientSessionEntity,
    ) -> Result<Client, CustomFailure> {
        println!("RUST: building client");

        match Client::builder()
            .server_name_or_homeserver_url(&client_session.homeserver)
            // We use the SQLite store, which is enabled by default. This is the crucial part to
            // persist the encryption setup.
            // Note that other store backends are available and you can even implement your own.
            .sqlite_store(
                &client_session.session_path,
                Some(&client_session.passphrase),
            )
            .build()
            .await
        {
            Ok(client) => return Ok(client),
            Err(error) => match &error {
                matrix_sdk::ClientBuildError::AutoDiscovery(_)
                | matrix_sdk::ClientBuildError::Url(_)
                | matrix_sdk::ClientBuildError::Http(_) => {
                    println!("RUST: Error checking the homeserver: {error}");
                    println!("RUST: Please try again\n");
                    Err(CustomFailure::NetworkError(
                        "Unable to validate homeserver".to_string(),
                    ))
                    // return here
                }
                other_error => {
                    // Use format! to turn the error into a String
                    let error_text = format!(
                        "An unexpected error occurred building matrix client: {}",
                        other_error
                    );

                    // Return the CustomFailure with the text inside
                    return Err(CustomFailure::Unknown(error_text));
                }
            },
        }
    }
}
