// src/domain/repositories/profile_repository.rs
use crate::core::error::failure::CustomFailure;
use crate::frb_generated::StreamSink;

pub trait SyncRepository {
    async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure>;
    async fn sync(&self, sink: StreamSink<String>) -> Result<(), CustomFailure>;
}
