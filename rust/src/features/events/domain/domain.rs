use anyhow::Result;
use matrix_sdk::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    // inbound from transport
    MessageReceived {
        address: Address,
        body: String,
    },

    // internal AI result
    AiResponseGenerated {
        address: Address,
        actions: Vec<ConversationAction>,
    },

    // outbound request
    ConversationActionsRequested {
        address: Address,
        actions: Vec<ConversationAction>,
    },

    // optional audit event
    ConversationActionsCompleted {
        address: Address,
    },
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
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: DomainEvent);
}

// Addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Address {
    ConversationId(Uuid),
    ParticipantId(Uuid),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationAction {
    SendText {
        text: String,
    },
    SendImage {
        url: String,
        caption: Option<String>,
    },
    ReactToMessage {
        message_id: String,
        emoji: String,
    },
}
pub struct ResolvedConversation {
    pub conversation_id: Uuid,
    pub room_id: String,
}

#[async_trait]
pub trait ConversationGateway: Send + Sync {
    async fn execute_actions(&self, address: Address, actions: Vec<ConversationAction>);
}
#[async_trait]
pub trait ConversationResolver: Send + Sync {
    async fn resolve(&self, address: Address) -> Result<ResolvedConversation>;
}
