use std::sync::Arc;

use crate::features::rooms::domain::{
    entities::room::RoomEntity, repositories::room_repository::RoomRepository,
};
use crate::frb_generated::StreamSink;
use anyhow::{Context, Result};

pub struct GetRooms<R: RoomRepository> {
     repo: Arc<R>,
}

impl<R: RoomRepository> GetRooms<R>
where
    R: RoomRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, space_id: String, sink: StreamSink<Vec<RoomEntity>>) -> Result<()> {
        self
            .repo
            .get_rooms_by_space(space_id, sink)
            .await
            .with_context(|| format!("Usecase: Failed to get rooms"))?;

        Ok(())
    }
}
