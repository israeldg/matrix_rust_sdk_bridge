use std::sync::Arc;

use matrix_sdk::async_trait;

use crate::features::events::domain::{
    DomainEvent, EventHandler, EventRepository, LlmClient, MatrixAdapter,
};
pub struct ProcessAiUseCase {
    pub llm: Arc<dyn LlmClient>,
    pub event_repo: Arc<dyn EventRepository>,
}
impl ProcessAiUseCase {
    pub async fn execute(&self, room_id: String, text: String) {
        let completion = self.llm.generate(&text).await;
        self.event_repo
            .persist_and_broadcast(DomainEvent::AiResponseGenerated {
                room_id,
                completion,
            })
            .await;
    }
}

pub struct SendReplyUseCase {
    pub adapter: Arc<dyn MatrixAdapter>,
    pub event_repo: Arc<dyn EventRepository>,
}
impl SendReplyUseCase {
    pub async fn execute(&self, room_id: String, completion: String) {
        self.adapter.send_message(&room_id, &completion).await;
        self.event_repo
            .persist_and_broadcast(DomainEvent::MatrixReplySent { room_id })
            .await;
    }
}

pub struct AiHandler {
    pub use_case: Arc<ProcessAiUseCase>,
}
#[async_trait]
impl EventHandler for AiHandler {
    async fn handle(&self, event: DomainEvent) {
        if let DomainEvent::MatrixMessageReceived { room_id, body } = event {
            println!("🧠 [Handler] Processing Matrix message...");
            self.use_case.execute(room_id, body).await;
        }
    }
}

pub struct MatrixReplyHandler {
    pub use_case: Arc<SendReplyUseCase>,
}
#[async_trait]
impl EventHandler for MatrixReplyHandler {
    async fn handle(&self, event: DomainEvent) {
        if let DomainEvent::AiResponseGenerated {
            room_id,
            completion,
        } = event
        {
            println!("💬 [Handler] Processing AI response...");
            self.use_case.execute(room_id, completion).await;
        }
    }
}
