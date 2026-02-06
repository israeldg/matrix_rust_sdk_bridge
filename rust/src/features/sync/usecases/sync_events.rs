use std::sync::Arc;

use crate::features::sync::domain::{
    repositories::sync_repository::SyncRepository,
};
use crate::frb_generated::StreamSink;
use anyhow::{Context, Result};

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

    pub async fn execute(&self, sink: StreamSink<String>) -> Result<()> {
        self.repo
            .sync(sink)
            .await
            .with_context(|| format!("Usecase: Failed to sync"))?;
        Ok(())
    }
}
