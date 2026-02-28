// errores de dominio sin dependencias de framework (actix, sqlx, etc)
// el mapeo a http se hace en el adaptador de errores

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("error de validación: {0}")]
    Validation(String),

    #[error("entidad no encontrada")]
    NotFound,

    #[error("credenciales inválidas")]
    AuthenticationFailed,

    #[error("entidad duplicada: {0}")]
    Duplicate(String),

    #[error("error interno: {0}")]
    Internal(String),
}

// Conversión desde sqlx para usar en adaptadores de persistencia
impl From<sqlx::Error> for DomainError {
    fn from(e: sqlx::Error) -> Self {
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
}