// src/domain/repositories/profile_repository.rs
use crate::{
    core::error::failure::CustomFailure,
    features::matrix_client_registry::domain::entities::registry_session::UserSessionEntity,
};

pub trait AuthRepository {
    // async fn login(&self, home_server: &String,
    //     data_dir: &Path,
    //     auth_token: &str) -> Result<MatrixSessionEntity, CustomFailure>;
    async fn restore_session(
        &self,
        user_session: UserSessionEntity,
    ) -> Result<UserSessionEntity, CustomFailure>;
    async fn logout(&self) -> Result<(), CustomFailure>;

    async fn login_with_password(
        &self,
        username: String,
        password: String,
    ) -> Result<UserSessionEntity, CustomFailure>;
}
