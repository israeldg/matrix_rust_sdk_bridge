use crate::{
    core::error::failure::CustomFailure,
    features::{
        auth::{
            data::datasources::auth_remote_data_source::AuthRemoteDataSource,
            domain::repositories::auth_repository::AuthRepository,
        },
        matrix_client_registry::{
            data::models::registry_session_model::UserSessionModel,
            domain::entities::registry_session::UserSessionEntity,
        },
    },
};

pub struct AuthRepositoryImpl<R: AuthRemoteDataSource> {
    remote: R,
}

impl<R: AuthRemoteDataSource> AuthRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: AuthRemoteDataSource> AuthRepository for AuthRepositoryImpl<R> {
    async fn logout(&self) -> Result<(), CustomFailure> {
        todo!()
    }

    async fn login_with_password(
        &self,
        username: String,
        password: String,
    ) -> Result<UserSessionEntity, CustomFailure> {
        Ok(self
            .remote
            .login_with_password(&username, &password)
            .await?
            .to_entity())
    }

    async fn restore_session(
        &self,
        user_session: UserSessionEntity,
    ) -> Result<UserSessionEntity, CustomFailure> {
        Ok(self
            .remote
            .restore_matrix_session(UserSessionModel::new(user_session))
            .await?
            .to_entity())
    }
}
