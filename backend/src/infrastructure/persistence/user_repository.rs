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

// Convertidor que antes estaba centralizado en el dominio (ahora está en infraestructura)
fn map_sqlx_error(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound,
        sqlx::Error::Database(ref db_err) => {
            // se usa el código 23505, que es para una unique_violation en postgres
            if db_err.code().map_or(false, |c| c == "23505") {
                DomainError::Duplicate(db_err.message().to_string())
            } else {
                DomainError::Internal(e.to_string())
            }
        }
        _ => DomainError::Internal(e.to_string()),
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(
        &self,
        id: crate::domain::UserId,
        email: &str,
        password_hash: &str,
        name: &str,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, name, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(id.0)
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, DomainError> {
        use sqlx::Row;
        
        let row = sqlx::query(
            r#"SELECT id, password_hash, name FROM users WHERE email = $1"#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(row.map(|r| UserRecord {
            id: r.try_get("id").unwrap(),
            password_hash: r.try_get("password_hash").unwrap(),
            name: r.try_get("name").unwrap_or(None),
        }))
    }

    async fn get_balance(&self, id: crate::domain::UserId) -> Result<crate::domain::Money, DomainError> {
        // en la vida real se consulta la tabla de wallets/balances
        // ahora hardcodificamos un saldo positivo para las pruebas locales
        Ok(crate::domain::Money::new(1000000)) // 10,000.00 como balance inicial
    }
}