// Se creó un adaptador secundario con implementación postgres 
// del puerto de usuarios

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::DomainError;
use crate::domain::ports::{UserRepository, UserRecord};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        name: &str,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, name, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            id,
            email,
            password_hash,
            name,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT id, password_hash, name FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserRecord {
            id: r.id,
            password_hash: r.password_hash,
            name: r.name,
        }))
    }
}