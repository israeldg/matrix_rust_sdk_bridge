use crate::features::rooms::domain::repositories::room_repository::RoomRepository;
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct SendMessageToRoom<R: RoomRepository> {
    repo: Arc<R>,
}

impl<R: RoomRepository> SendMessageToRoom<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, room_id: String, message_content: String) -> Result<()> {
        self.repo
            .send_message_to_room(room_id, message_content)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string())) // Convert CustomFailure to anyhow
            .with_context(|| "Usecase: Failed to get room stream")?;

        Ok(())
    }
}
