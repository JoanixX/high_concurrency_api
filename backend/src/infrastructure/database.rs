// Se hizo un pool de conexiones a postgres
// es solo nfraestructura pura para el wiring de lib.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use secrecy::ExposeSecret;
use crate::config::DatabaseSettings;

pub async fn build_connection_pool(
    configuration: &DatabaseSettings,
) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(100)
        .min_connections(5)
        .idle_timeout(std::time::Duration::from_secs(30))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect_with(
            configuration
                .connection_string()
                .expose_secret()
                .parse()
                .unwrap(),
        )
        .await?;

    // migraciones automáticas al iniciar
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Fallo la ejecución de las migraciones: {:?}", e);
            e
        })?;

    Ok(pool)
}