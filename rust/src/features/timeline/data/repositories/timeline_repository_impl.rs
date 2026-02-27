use crate::{
    core::error::failure::CustomFailure,
    features::timeline::{
        data::datasources::timeline_remote_data_source::TimelineRemoteDataSource,
        domain::{
            entities::event_entity_delta::EventDeltaEntity,
            repositories::timeline_repository::TimelineRepository,
        },
    },
};
use futures_util::stream::BoxStream;
pub struct TimelineRepositoryImpl<R: TimelineRemoteDataSource> {
    remote: R,
}

impl<R: TimelineRemoteDataSource> TimelineRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: TimelineRemoteDataSource> TimelineRepository for TimelineRepositoryImpl<R> {
    async fn fetch_events_by_room_id(
        &self,
        room_id: String,
    ) -> Result<BoxStream<'static, Vec<EventDeltaEntity>>, CustomFailure> {
        self.remote.fetch_events_by_room_id(room_id).await
    }
}
