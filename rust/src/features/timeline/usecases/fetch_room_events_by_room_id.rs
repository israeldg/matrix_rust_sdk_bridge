use std::sync::Arc;

use crate::{
    features::timeline::domain::{
        entities::{event::EventEntity, event_entity_delta::EventDeltaEntity},
        repositories::timeline_repository::TimelineRepository,
    },
    frb_generated::StreamSink,
};
use anyhow::Result;

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
        sink: StreamSink<Vec<EventDeltaEntity>>,
    ) -> Result<()> {
        Ok(self.repo.fetch_events_by_room_id(room_id, sink).await?)
    }
}
