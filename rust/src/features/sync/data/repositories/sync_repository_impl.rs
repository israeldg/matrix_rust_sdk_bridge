use crate::core::error::failure::CustomFailure;
use crate::features::sync::data::datasources::sync_remote_data_source::SyncRemoteDataSource;
use crate::features::sync::domain::repositories::sync_repository::SyncRepository;
use crate::frb_generated::StreamSink;

pub struct SyncRepositoryImpl<R: SyncRemoteDataSource> {
    remote: R,
}

impl<R: SyncRemoteDataSource> SyncRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: SyncRemoteDataSource> SyncRepository for SyncRepositoryImpl<R> {
    async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure> {
        todo!()
    }

    async fn sync(&self, sink: StreamSink<String>) -> Result<(), CustomFailure> {
        self.remote.sync(sink).await
    }
}
