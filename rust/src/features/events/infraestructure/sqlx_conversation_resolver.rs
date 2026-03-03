use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::{
    core::common::matrix_client_management::matrix_client_context::MatrixClientContext,
    features::events::domain::domain::{Address, ConversationResolver, ResolvedConversation},
};

pub struct SqlxConversationResolver {
    pub pool: PgPool,
    pub matrix_client: Arc<MatrixClientContext>,
}

#[async_trait]
impl ConversationResolver for SqlxConversationResolver {
    async fn resolve(&self, address: Address) -> Result<ResolvedConversation> {
        match address {
            Address::ConversationId(id) => self.resolve_by_conversation_id(id).await,
            Address::ParticipantId(pid) => todo!(),
        }
    }
}

impl SqlxConversationResolver {
    async fn resolve_by_conversation_id(
        &self,
        conversation_id: Uuid,
    ) -> Result<ResolvedConversation> {
        let record = sqlx::query("SELECT room_id FROM conversations WHERE id = $1")
            .bind(conversation_id)
            .fetch_optional(&self.pool)
            .await?;

        let row = record.ok_or_else(|| anyhow!("Conversation not found"))?;

        let room_id: String = row.get("room_id");

        Ok(ResolvedConversation {
            conversation_id,
            room_id,
        })
    }
}
