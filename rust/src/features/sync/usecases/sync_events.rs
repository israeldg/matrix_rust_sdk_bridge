use std::sync::Arc;

use crate::features::sync::domain::repositories::sync_repository::SyncRepository;
use anyhow::{Context, Result};
use futures_util::stream::BoxStream;

pub struct SyncEvents<R: SyncRepository> {
    repo: Arc<R>,
}

impl<R: SyncRepository> SyncEvents<R>
where
    R: SyncRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<BoxStream<'static, String>> {
        let stream = self
            .repo
            .sync()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string())) // Convert CustomFailure to anyhow
            .with_context(|| "Usecase: Failed to Sync")?;

        Ok(stream)
    }
}
