// src/data/repositories/room_repository_impl.rs
use crate::{
    core::error::failure::CustomFailure,
    features::rooms::{
        data::datasources::room_remote_data_source::RoomRemoteDataSource,
        domain::{entities::room::RoomEntity, repositories::room_repository::RoomRepository},
    },
};
use futures_util::stream::BoxStream;

pub struct RoomRepositoryImpl<R: RoomRemoteDataSource> {
    remote: R,
}

impl<R: RoomRemoteDataSource> RoomRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: RoomRemoteDataSource> RoomRepository for RoomRepositoryImpl<R> {
    async fn get_rooms_by_space(
        &self,
        space_id: String,
    ) -> Result<BoxStream<'static, Vec<RoomEntity>>, CustomFailure> {
        // Just forward the stream from the remote data source
        self.remote.get_rooms_by_space_stream(space_id).await
    }

    async fn get_spaces(&self) -> Result<Vec<RoomEntity>, CustomFailure> {
        match self.remote.get_spaces().await {
            Ok(models) => {
                let entities = models.into_iter().map(|m| m.to_entity()).collect();
                Ok(entities)
            }
            Err(e) => Err(CustomFailure::NotFound(e.to_string())),
        }
    }
}
