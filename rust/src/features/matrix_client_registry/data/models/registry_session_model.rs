use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::ruma::{device_id, DeviceId, UserId};
use matrix_sdk::{SessionMeta, SessionTokens};
use serde::{Deserialize, Serialize};

use crate::core::error::failure::CustomFailure;
use crate::features::matrix_client_registry::domain::entities::registry_session::UserSessionEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionModel {
    #[serde(flatten)] // optional — merges entity fields into same JSON structure
    pub entity: UserSessionEntity,
}

impl UserSessionModel {
    pub fn new(entity: UserSessionEntity) -> Self {
        Self { entity }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.entity).unwrap()
    }

    pub fn to_entity(self) -> UserSessionEntity {
        self.entity
    }

    /// Construct a `UserSessionModel` from a `MatrixSession`
    pub fn from_matrix_session(matrix_session: MatrixSession) -> Result<Self, CustomFailure> {
        let entity: UserSessionEntity = UserSessionEntity {
            access_token: matrix_session.tokens.access_token,
            refresh_token: matrix_session.tokens.refresh_token,
            matrix_user_id: matrix_session.meta.user_id.to_string(),
            device_id: matrix_session.meta.device_id.to_string(),
        };

        Ok(Self { entity })
    }
    pub fn to_matrix_session(&self) -> Result<MatrixSession, CustomFailure> {
        let user_id = UserId::parse(&self.entity.matrix_user_id)
            .map_err(|e| CustomFailure::InvalidInput(format!("Invalid user ID: {}", e)))?;

        let matrix_session = MatrixSession {
            tokens: SessionTokens {
                access_token: self.entity.access_token.to_owned(),
                refresh_token: self.entity.refresh_token.to_owned(),
            },
            meta: SessionMeta {
                user_id,
                device_id: self.entity.device_id.to_owned().into(),
            },
        };

        Ok(matrix_session)
    }
}
