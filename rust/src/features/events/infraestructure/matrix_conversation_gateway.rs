use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    core::common::matrix_client_management::matrix_client_context::MatrixClientContext,
    features::events::domain::domain::{
        Address, ConversationAction, ConversationGateway, ConversationResolver,
    },
};

pub struct MatrixConversationGateway {
    pub resolver: Arc<dyn ConversationResolver>,
    pub matrix_client: Arc<MatrixClientContext>,
}

#[async_trait]
impl ConversationGateway for MatrixConversationGateway {
    async fn execute_actions(&self, address: Address, actions: Vec<ConversationAction>) {
        let resolved = match self.resolver.resolve(address).await {
            Ok(r) => r,
            Err(e) => {
                println!("Resolver error: {:?}", e);
                return;
            }
        };

        for action in actions {
            match action {
                ConversationAction::SendText { text } => {
                    let _ = self
                        .matrix_client
                        .send_message_to_room
                        .execute(resolved.room_id.clone(), text)
                        .await;
                }

                ConversationAction::SendImage { .. } => {
                    println!("SendImage not implemented yet");
                }

                ConversationAction::ReactToMessage { .. } => {
                    println!("ReactToMessage not implemented yet");
                }
            }
        }
    }
}
