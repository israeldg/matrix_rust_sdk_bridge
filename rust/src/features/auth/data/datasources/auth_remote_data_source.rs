// src/infrastructure/datasources/profile_remote_data_source.rs

use matrix_sdk::Client;

use crate::{
    core::error::failure::CustomFailure,
    features::matrix_client_registry::data::models::registry_session_model::UserSessionModel,
};

pub trait AuthRemoteDataSource {
    // async fn login(
    //     &self,
    //     home_server: &String,
    //     data_dir: &Path,
    //     auth_token: &str,
    // ) -> Result<(Client, MatrixSessionEntity), CustomFailure>;
    async fn restore_matrix_session(
        &self,
        user_session: UserSessionModel,
    ) -> Result<UserSessionModel, CustomFailure>;
    async fn logout(&self) -> Result<(), CustomFailure>;
    async fn login_with_password(
        &self,
        username: &String,
        password: &String,
    ) -> Result<UserSessionModel, CustomFailure>;
}

#[derive(Clone)]
pub struct AuthRemoteDataSourceImpl {
    matrix_client: Client,
}

impl AuthRemoteDataSourceImpl {
    pub fn new(matrix_client: Client) -> Self {
        Self { matrix_client }
    }
}

impl AuthRemoteDataSource for AuthRemoteDataSourceImpl {
    async fn restore_matrix_session(
        &self,
        user_session: UserSessionModel,
    ) -> Result<UserSessionModel, CustomFailure> {
        // Restore the Matrix user session.

        let matrix_session = user_session
            .to_matrix_session()
            .expect("unable to parse matrix session");

        self.matrix_client
            .restore_session(matrix_session)
            .await
            .map_err(|e| CustomFailure::InvalidInput(e.to_string()))?;

        Ok(user_session)
    }

    async fn logout(&self) -> Result<(), CustomFailure> {
        todo!()
    }

    /// Asks the user of a username and password, and try to login using the matrix
    /// auth with those.
    async fn login_with_password(
        &self,
        username: &String,
        password: &String,
    ) -> Result<UserSessionModel, CustomFailure> {
        let matrix_auth = self.matrix_client.matrix_auth();

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
}
