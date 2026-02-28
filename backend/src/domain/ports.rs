// traits que definen las interfaces, el dominio y los casos
// de uso dependen de los traits y las implementaciones 
// concretas van en la carpeta infrastructure

use async_trait::async_trait;
use uuid::Uuid;
use super::models::{BetTicket, User, BetStatus};
use super::errors::DomainError;

// Puerto de apuestas
#[async_trait]
pub trait BetRepository: Send + Sync {
    async fn save(
        &self,
        id: Uuid,
        ticket: &BetTicket,
        status: &BetStatus,
    ) -> Result<(), DomainError>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<BetTicket>, DomainError>;
}

// Puerto de usuarios
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        name: &str,
    ) -> Result<(), DomainError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, DomainError>;
}

// registro devuelto por el repositorio con hash
#[derive(Debug)]
pub struct UserRecord {
    pub id: Uuid,
    pub password_hash: String,
    pub name: Option<String>,
}

// Puerto de cache
#[async_trait]
pub trait CachePort: Send + Sync {
    async fn set(&self, key: &str, value: &str, expire_secs: usize) -> Result<(), DomainError>;
    async fn get(&self, key: &str) -> Result<Option<String>, DomainError>;
}

// Puerto de hashing de contraseñas
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, DomainError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}