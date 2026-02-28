// Login de usuario
// verifica credenciales usando puertos, sin conocer argon2 ni postgres

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::DomainError;
use crate::domain::ports::{UserRepository, PasswordHasher};

pub struct LoginUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

#[derive(Debug)]
pub struct LoginResult {
    pub user_id: Uuid,
    pub name: Option<String>,
}

impl LoginUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self { user_repo, hasher }
    }

    pub async fn execute(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginResult, DomainError> {
        // buscar usuario vía puerto
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or(DomainError::AuthenticationFailed)?;

        // verificar contraseña vía puerto
        let is_valid = self.hasher.verify(password, &user.password_hash)?;
        if !is_valid {
            return Err(DomainError::AuthenticationFailed);
        }

        tracing::info!(user_id = %user.id, "Login exitoso");

        Ok(LoginResult {
            user_id: user.id,
            name: user.name,
        })
    }
}