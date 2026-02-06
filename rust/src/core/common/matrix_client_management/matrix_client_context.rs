use matrix_sdk::Client;
use matrix_sdk_ui::sync_service::SyncService;
use std::sync::Arc;

use crate::features::{
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
    rooms::{
        data::{
            datasources::room_remote_data_source::RoomRemoteDataSourceImpl,
            repositories::room_repository_impl::RoomRepositoryImpl,
        },
        usecases::get_rooms::GetRooms,
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
        usecases::fetch_room_events_by_room_id::FetchRoomEventsByRoomId,
    },
};

pub struct MatrixClientContext {
    pub client: Client,
    pub sync_service: Arc<SyncService>,
    //pub timeline: Arc<TimelineService>,

    //UseCases
    //pub login_matrix: Arc<LoginMatrix<AuthRepositoryImpl<AuthRemoteDataSourceImpl>>>,
    pub login_matrix_with_password:
        Arc<LoginMatrixWithPassword<AuthRepositoryImpl<AuthRemoteDataSourceImpl>>>,
    pub restore_matrix_session:
        Arc<RestoreMatrixSession<AuthRepositoryImpl<AuthRemoteDataSourceImpl>>>,
    pub sync_events: Arc<SyncEvents<SyncRepositoryImpl<SyncRemoteDataSourceImpl>>>,

    pub get_rooms: Arc<GetRooms<RoomRepositoryImpl<RoomRemoteDataSourceImpl>>>,
    pub fetch_room_events_by_room_id:
        Arc<FetchRoomEventsByRoomId<TimelineRepositoryImpl<TimelineRemoteDataSourceImpl>>>,
}
