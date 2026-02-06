use crate::{
    core::error::failure::CustomFailure,
    features::timeline::{
        data::datasources::timeline_remote_data_source::TimelineRemoteDataSource,
        domain::{
            entities::{event::EventEntity, event_entity_delta::EventDeltaEntity},
            repositories::timeline_repository::TimelineRepository,
        },
    },
    frb_generated::StreamSink,
};

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
        sink: StreamSink<Vec<EventDeltaEntity>>,
    ) -> Result<(), CustomFailure> {
        self.remote.fetch_events_by_room_id(room_id, sink).await
    }
}
