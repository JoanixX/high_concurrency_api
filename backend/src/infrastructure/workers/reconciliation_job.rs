// job de reconciliacion de balances entre postgres y 
// redis que es el cache caliente
// para recorrer todos los registros se usa el keyset pagination
// y para consultar redis sin tantos viajes se usa el mget/pipeline

use deadpool_redis::Pool as RedisPool;
use deadpool_redis::redis::AsyncCommands;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::{info, error, warn};
use tokio_cron_scheduler::{JobScheduler, Job};

const BATCH_SIZE: i64 = 1000;

pub async fn start_reconciliation_scheduler(
    cron_expression: &str,
    redis_pool: RedisPool,
    db_pool: PgPool,
) -> Result<JobScheduler, anyhow::Error> {
    let sched = JobScheduler::new().await?;

    let cron_expr = cron_expression.to_string();
    let rp = redis_pool.clone();
    let dp = db_pool.clone();

    let job = Job::new_async(cron_expr.as_str(), move |_uuid, _lock| {
        let redis_pool = rp.clone();
        let db_pool = dp.clone();
        Box::pin(async move {
            info!("Iniciando job de reconciliación de balances...");
            if let Err(e) = run_reconciliation(&redis_pool, &db_pool).await {
                error!("Error en el job de reconciliación: {:?}", e);
            }
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;

    info!("Scheduler de reconciliación iniciado con cron: {}", cron_expression);
    Ok(sched)
}

async fn run_reconciliation(
    redis_pool: &RedisPool,
    db_pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut redis_conn = redis_pool.get().await?;
    let mut last_id = Uuid::nil();
    let mut total_checked: u64 = 0;
    let mut total_fixed: u64 = 0;

    loop {
        // keyset pagination: evita el offset que es o(n) en postgres
        use sqlx::Row;
        let rows = sqlx::query(
            r#"
            SELECT id, balance 
            FROM users 
            WHERE id > $1 
            ORDER BY id ASC 
            LIMIT $2
            "#
        )
        .bind(last_id)
        .bind(BATCH_SIZE)
        .fetch_all(db_pool)
        .await?;

        if rows.is_empty() {
            break;
        }

        let mut user_ids: Vec<Uuid> = Vec::with_capacity(rows.len());
        let mut db_balances: Vec<i64> = Vec::with_capacity(rows.len());

        for row in &rows {
            let uid: Uuid = row.try_get("id")?;
            let balance: i64 = row.try_get("balance")?;
            user_ids.push(uid);
            db_balances.push(balance);
        }

        last_id = *user_ids.last().unwrap();

        // construimos las claves de redis para el mget
        let redis_keys: Vec<String> = user_ids.iter()
            .map(|uid| format!("user:{}:balance", uid))
            .collect();

        // el mget hace un solo viaje para hacer las n consultas a redis
        let redis_values: Vec<Option<i64>> = redis_conn.get(&redis_keys[..]).await?;

        // comparamos y corregimos discrepancias con un pipeline
        let mut pipe = deadpool_redis::redis::pipe();
        let mut pipe_has_commands = false;

        for (i, redis_val) in redis_values.iter().enumerate() {
            let db_balance = db_balances[i];
            let user_id = user_ids[i];

            match redis_val {
                Some(redis_balance) => {
                    if *redis_balance != db_balance {
                        error!(
                            "DISCREPANCIA DETECTADA: user_id={}, balance_db={}, balance_redis={}. Sobreescribiendo Redis.",
                            user_id, db_balance, redis_balance
                        );
                        pipe.set(&redis_keys[i], db_balance).ignore();
                        pipe_has_commands = true;
                        total_fixed += 1;
                    }
                }
                // si el usuario no tiene clave en redis no se crea
                None => {}
            }
        }
        // ejecutamos el pipeline solo si hay correcciones
        if pipe_has_commands {
            let pipe_res: deadpool_redis::redis::RedisResult<()> = pipe.query_async(&mut *redis_conn).await;
            if let Err(e) = pipe_res {
                error!("Fallo al ejecutar pipeline de corrección en Redis: {:?}", e);
            }
        }
        total_checked += user_ids.len() as u64;

        // si el lote fue menor al batch size, ya no hay mas paginas
        if (rows.len() as i64) < BATCH_SIZE {
            break;
        }
    }

    if total_fixed > 0 {
        warn!(
            "Reconciliación completada: {} usuarios verificados, {} discrepancias corregidas",
            total_checked, total_fixed
        );
    } else {
        info!(
            "Reconciliación completada: {} usuarios verificados, sin discrepancias",
            total_checked
        );
    }

    Ok(())
}