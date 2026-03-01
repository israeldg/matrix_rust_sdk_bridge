// src/domain/repositories/profile_repository.rs
use crate::{
    core::error::failure::CustomFailure,
    features::sync::domain::entities::simple_event::SimpleEvent,
};

use futures_util::stream::BoxStream;

pub trait SyncRepository {
    //async fn sync_once(&self, initial_sync_token: Option<String>) -> Result<String, CustomFailure>;
    async fn sync(&self) -> Result<BoxStream<'static, SimpleEvent>, CustomFailure>;
}
