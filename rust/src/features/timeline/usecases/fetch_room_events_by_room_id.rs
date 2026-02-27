use std::sync::Arc;

use crate::features::timeline::domain::{
    entities::event_entity_delta::EventDeltaEntity,
    repositories::timeline_repository::TimelineRepository,
};
use anyhow::{Context, Result};
use futures_util::stream::BoxStream;

pub struct FetchRoomEventsByRoomId<R: TimelineRepository> {
    repo: Arc<R>,
}

impl<R: TimelineRepository> FetchRoomEventsByRoomId<R>
where
    R: TimelineRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        room_id: String,
    ) -> Result<BoxStream<'static, Vec<EventDeltaEntity>>> {
        let stream = self
            .repo
            .fetch_events_by_room_id(room_id)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string())) // Convert CustomFailure to anyhow
            .with_context(|| "Usecase: Failed to get room stream")?;

        Ok(stream)
    }
}
