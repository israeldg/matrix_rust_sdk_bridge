use crate::core::error::failure::CustomFailure;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;

use futures_util::stream::BoxStream;
pub trait TimelineRepository {
    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
    ) -> Result<BoxStream<'static, Vec<EventDeltaEntity>>, CustomFailure>;
}
