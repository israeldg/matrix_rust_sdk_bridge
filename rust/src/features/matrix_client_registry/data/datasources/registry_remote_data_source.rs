// src/infrastructure/datasources/profile_remote_data_source.rs

use std::sync::Arc;

use futures_util::future::err;
use matrix_sdk::Client;
use matrix_sdk_ui::sync_service::SyncService;

use crate::{
    core::{
        common::matrix_client_management::{
            matrix_client_context_factory::MatrixClientContextFactory,
            matrix_client_registry::MatrixClientRegistry,
        },
        error::failure::CustomFailure,
    },
    features::matrix_client_registry::{
        data::models::registry_session_model::UserSessionModel,
        domain::entities::registry_session::{Credentials, MatrixSessionEntity, UserSessionEntity},
    },
};

pub trait RegistryRemoteDataSource {
    async fn register_client(
        &self,
        full_session: MatrixSessionEntity,
    ) -> Result<MatrixSessionEntity, CustomFailure>;
}

#[derive(Clone)]
pub struct RegistryRemoteDataSourceImpl {
    registry: Arc<MatrixClientRegistry>,
    client_factory: Arc<MatrixClientContextFactory>,
}

impl RegistryRemoteDataSourceImpl {
    pub fn new(
        registry: Arc<MatrixClientRegistry>,
        client_factory: Arc<MatrixClientContextFactory>,
    ) -> Self {
        Self {
            registry: registry,
            client_factory: client_factory,
        }
    }
    async fn login_with_password(
        &self,
        matrix_client: &Client,
        username: &String,
        password: &String,
    ) -> Result<UserSessionModel, CustomFailure> {
        let matrix_auth = matrix_client.matrix_auth();

        match matrix_auth.login_username(&username, password.trim()).await {
            Ok(_) => {
                println!("RUST: Logged in as {username}");
            }
            Err(error) => {
                println!("RUST: Error logging in: {error}");
                println!("RUST: Please try again\n");
                return Err(CustomFailure::Unknown(
                    "Unable to authenticate Matrix using password".to_string(),
                ));
            }
        }

        // Persist the session to reuse it later.
        // This is not very secure, for simplicity. If the system provides a way of
        // storing secrets securely, it should be used instead.
        // Note that we could also build the user session from the login response.
        let user_session = matrix_auth
            .session()
            .expect("A logged-in client should have a session");

        // After logging in, you might want to verify this session with another one (see
        // the `emoji_verification` example), or bootstrap cross-signing if this is your
        // first session with encryption, or if you need to reset cross-signing because
        // you don't have access to your old sessions (see the
        // `cross_signing_bootstrap` example).

        match UserSessionModel::from_matrix_session(user_session) {
            Ok(model) => Ok(model),
            Err(e) => {
                return Err(CustomFailure::InvalidInput(format!(
                    "Invalid session, not able to authenticate: {}",
                    e
                )))
            }
        }
    }

    async fn restore_matrix_session(
        &self,
        matrix_client: &Client,
        user_session: UserSessionModel,
    ) -> Result<UserSessionModel, CustomFailure> {
        // Restore the Matrix user session.

        let matrix_session = user_session
            .to_matrix_session()
            .expect("unable to parse matrix session");

        matrix_client
            .restore_session(matrix_session)
            .await
            .map_err(|e| CustomFailure::InvalidInput(e.to_string()))?;

        Ok(user_session)
    }
}

impl RegistryRemoteDataSource for RegistryRemoteDataSourceImpl {
    async fn register_client(
        &self,
        full_session: MatrixSessionEntity,
    ) -> Result<MatrixSessionEntity, CustomFailure> {
        let MatrixSessionEntity {
            client_session,
            user_session,
            sync_token: _,
            credentials,
        } = full_session;

        println!(
            "RUST: Attempting connection to: {}",
            client_session.homeserver
        );

        //1. Build Matrix Client
        let matrix_client = self.client_factory.build_client(&client_session).await?;

        //2. Login or restore session
        let user_session: UserSessionEntity = if let Some(session) = user_session {
            let user_session_model = UserSessionModel::new(session);
            let logged_session = self
                .restore_matrix_session(&matrix_client, user_session_model)
                .await?;
            logged_session.to_entity()
        } else {
            if let Some(cred) = &credentials {
                match cred {
                    Credentials::AccessToken(token) => {
                        return Err(CustomFailure::InvalidInput(
                            "Token Not Supported, TODO".to_string(),
                        ));
                    }
                    Credentials::UserPassword { username, password } => {
                        let logged_session = self
                            .login_with_password(&matrix_client, username, password)
                            .await?;

                        logged_session.to_entity()
                    }
                }
            } else {
                return Err(CustomFailure::InvalidInput(
                    "invalid credentials".to_string(),
                ));
            }
        };
        //3. Build Sync_service
        let sync_service = Arc::new(
            SyncService::builder(matrix_client.clone())
                .with_share_pos(true)
                .build()
                .await
                .map_err(|e| {
                    CustomFailure::Unknown(format!("Failed to build sync service: {:?}", e))
                })?,
        );

        //4. Build Matrix Client Context
        let client_context = self
            .client_factory
            .create(matrix_client, sync_service)
            .await;

        //5. Register client in the global registry IMPORTANT(account_id = matrix_user_id))
        self.registry.register_client(
            user_session.matrix_user_id.clone(),
            Arc::new(client_context),
        );
        //6. Set generated client as active client
        self.registry
            .set_active_account(user_session.matrix_user_id.clone())
            .unwrap();

        println!("RUST: AppContext Created");

        //7. Return full session client
        Ok(MatrixSessionEntity {
            client_session,
            user_session: Some(user_session),
            sync_token: None,
            credentials,
        })
    }
}
