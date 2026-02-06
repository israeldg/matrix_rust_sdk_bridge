// lib/features/auth/domain/entities/auth_session.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSessionEntity {
    /// The URL of the homeserver of the user.
    pub homeserver: String,

    /// The path of the database.
    pub session_path: String,

    /// The passphrase of the database.
    pub passphrase: String,
}

// Implement a constructor (optional, as field-init syntax works too)
impl ClientSessionEntity {
    pub fn new(homeserver: String, session_path: String, passphrase: String) -> Self {
        Self {
            homeserver,
            session_path,
            passphrase,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionEntity {
    // Rust uses `Option<T>` for optional/nullable fields.
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub device_id: String,
    pub matrix_user_id: String,
}

impl UserSessionEntity {
    pub fn new(
        access_token: String,
        refresh_token: Option<String>,
        matrix_user_id: String,
        device_id: String,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            matrix_user_id,
            device_id,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Credentials {
    AccessToken(String),
    UserPassword { username: String, password: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixSessionEntity {
    /// The data to re-build the client.
    pub client_session: ClientSessionEntity,

    /// The Matrix user session.
    pub user_session: Option<UserSessionEntity>,
    /// The latest sync token.
    ///
    /// It is only needed to persist it when using `Client::sync_once()` and we
    /// want to make our syncs faster by not receiving all the initial sync
    /// again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,

    pub credentials: Option<Credentials>,
}

impl MatrixSessionEntity {
    pub fn new(
        client_session: ClientSessionEntity,
        user_session: Option<UserSessionEntity>,
        sync_token: Option<String>,
        credentials: Option<Credentials>,
    ) -> Self {
        Self {
            client_session,
            user_session,
            sync_token,
            credentials,
        }
    }
}
