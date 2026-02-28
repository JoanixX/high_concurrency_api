// Registrar usuario
// depende solo de puertos, no de implementaciones concretas

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::DomainError;
use crate::domain::ports::{UserRepository, PasswordHasher};

pub struct RegisterUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

#[derive(Debug)]
pub struct RegisterResult {
    pub user_id: Uuid,
}

impl RegisterUserUseCase {
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
        name: &str,
    ) -> Result<RegisterResult, DomainError> {
        // Validaciones de dominio
        if email.is_empty() || !email.contains('@') {
            return Err(DomainError::Validation("email inválido".to_string()));
        }
        if password.len() < 8 {
            return Err(DomainError::Validation(
                "la contraseña debe tener al menos 8 caracteres".to_string(),
            ));
        }

        // hashear vía puerto
        let password_hash = self.hasher.hash(password)?;
        let user_id = Uuid::new_v4();

        // persistir vía puerto
        self.user_repo.save(user_id, email, &password_hash, name).await?;
        tracing::info!(user_id = %user_id, "usuario registrado");

        Ok(RegisterResult { user_id })
    }
}