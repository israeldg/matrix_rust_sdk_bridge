use std::sync::Arc;

use matrix_sdk::async_trait;

use crate::features::events::domain::domain::{
    ConversationAction, DomainEvent, EventHandler, EventRepository, LlmClient,
};

pub struct AiMessageHandler {
    pub llm: Arc<dyn LlmClient>,
    pub event_repository: Arc<dyn EventRepository>,
}
#[async_trait]
impl EventHandler for AiMessageHandler {
    async fn handle(&self, event: DomainEvent) {
        if let DomainEvent::MessageReceived { address, body } = event {
            let completion = self.llm.generate(&body).await;

            let actions = vec![ConversationAction::SendText { text: completion }];

            let response_event = DomainEvent::ConversationActionsRequested { address, actions };

            self.event_repository
                .persist_and_broadcast(response_event)
                .await;
        }
    }
}

// pub struct ProcessAiUseCase {
//     pub llm: Arc<dyn LlmClient>,
//     pub event_repo: Arc<dyn EventRepository>,
// }
// impl ProcessAiUseCase {
//     pub async fn execute(&self, room_id: String, text: String) {
//         let completion = self.llm.generate(&text).await;
//         // self.event_repo
//         //     .persist_and_broadcast(DomainEvent::AiResponseGenerated {
//         //         room_id,
//         //         completion,
//         //     })
//         //     .await;
//     }
// }

// pub struct AiHandler {
//     pub use_case: Arc<ProcessAiUseCase>,
// }
// #[async_trait]
// impl EventHandler for AiHandler {
//     async fn handle(&self, event: DomainEvent) {
//         if let DomainEvent::MatrixMessageReceived { room_id, body } = event {
//             println!("🧠 [Handler] Processing Matrix message...");
//             self.use_case.execute(room_id, body).await;
//         }
//     }
// }
