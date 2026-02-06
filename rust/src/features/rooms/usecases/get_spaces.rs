use std::sync::Arc;

use crate::features::rooms::domain::{
    entities::room::RoomEntity, repositories::room_repository::RoomRepository,
};
use anyhow::{Context, Result};

pub struct GetSpaces<R: RoomRepository> {
    repo: Arc<R>,
}

impl<R: RoomRepository> GetSpaces<R>
where
    R: RoomRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self) -> Result<Vec<RoomEntity>> {
        Ok(self
            .repo
            .get_spaces()
            .await
            .with_context(|| format!("Usecase: Failed to get spaces"))?)
    }
}
