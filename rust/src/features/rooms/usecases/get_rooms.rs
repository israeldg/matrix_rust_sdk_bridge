use crate::features::rooms::domain::{
    entities::room::RoomEntity, repositories::room_repository::RoomRepository,
};
use anyhow::{Context, Result};
use futures_util::stream::BoxStream;
use std::sync::Arc;

pub struct GetRooms<R: RoomRepository> {
    repo: Arc<R>,
}

impl<R: RoomRepository> GetRooms<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, space_id: String) -> Result<BoxStream<'static, Vec<RoomEntity>>> {
        let stream = self
            .repo
            .get_rooms_by_space(space_id)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string())) // Convert CustomFailure to anyhow
            .with_context(|| "Usecase: Failed to get room stream")?;

        Ok(stream)
    }
}
