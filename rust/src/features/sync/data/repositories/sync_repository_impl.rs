use crate::core::error::failure::CustomFailure;
use crate::features::sync::data::datasources::sync_remote_data_source::SyncRemoteDataSource;
use crate::features::sync::domain::repositories::sync_repository::SyncRepository;
use futures_util::stream::BoxStream;
pub struct SyncRepositoryImpl<R: SyncRemoteDataSource> {
    remote: R,
}

impl<R: SyncRemoteDataSource> SyncRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: SyncRemoteDataSource> SyncRepository for SyncRepositoryImpl<R> {
    // async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure> {
    //     todo!()
    // }

    async fn sync(&self) -> Result<BoxStream<'static, String>, CustomFailure> {
        self.remote.sync().await
    }
}
