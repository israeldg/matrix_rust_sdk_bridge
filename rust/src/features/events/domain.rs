use matrix_sdk::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    MatrixMessageReceived { room_id: String, body: String },
    AiResponseGenerated { room_id: String, completion: String },
    MatrixReplySent { room_id: String },
}
#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn persist_and_broadcast(&self, event: DomainEvent);
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> String;
}

#[async_trait]
pub trait MatrixAdapter: Send + Sync {
    async fn send_message(&self, room_id: &str, body: &str);
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: DomainEvent);
}
