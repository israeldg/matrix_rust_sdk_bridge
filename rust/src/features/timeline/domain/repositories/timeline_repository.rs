use crate::core::error::failure::CustomFailure;
use crate::features::timeline::domain::entities::event::EventEntity;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;
use crate::frb_generated::StreamSink;

pub trait TimelineRepository {
    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
        sink: StreamSink<Vec<EventDeltaEntity>>,
    ) -> Result<(), CustomFailure>;
}
