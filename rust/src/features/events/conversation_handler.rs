use std::sync::Arc;

use matrix_sdk::async_trait;

use crate::features::events::domain::domain::{ConversationGateway, DomainEvent, EventHandler};

pub struct ConversationActionHandler {
    pub gateway: Arc<dyn ConversationGateway>,
}
#[async_trait]
impl EventHandler for ConversationActionHandler {
    async fn handle(&self, event: DomainEvent) {
        if let DomainEvent::ConversationActionsRequested { address, actions } = event {
            self.gateway.execute_actions(address.clone(), actions).await;
        }
    }
}
