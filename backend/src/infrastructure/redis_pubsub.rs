use std::time::Duration;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::time::sleep;

use crate::domain::MatchId;
use crate::handlers::ws::manager::ConnectionManager;

// payload esperado del canal pub/sub de redis
#[derive(Deserialize, Debug)]
struct OddsUpdatePayload {
    match_id: String,
    new_odds: String,
}

// esto inicia una tarea asíncrona de fondo para escuchar actualizaciones en redis
// y tambien incluye lógica para reconexion infinita (resiliencia) 
// si el socket pub/sub cae
pub fn spawn_redis_pubsub_worker(redis_url: String, manager: ConnectionManager) {
    tokio::spawn(async move {
        // el loop exterior intenta mantener o reconectar el worker infinitamente
        loop {
            tracing::info!("Intentando conectar al pub/sub de Redis...");

            match connect_and_listen(&redis_url, &manager).await {
                Ok(_) => {
                    // si sale normalmente, simplemente se rompe
                    tracing::info!("Bucle pub/sub terminado limpiamente.");
                    break;
                }
                Err(e) => {
                    tracing::error!("Error en conexión pub/sub de Redis: {}. Reconectando en 5s...", e);
                    // backoff ligero para no spamear conexiones fallidas
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}

// aca va la logica interna de conexión y parsing
async fn connect_and_listen(redis_url: &str, manager: &ConnectionManager) -> anyhow::Result<()> {
    // se crea una conexion dedicada puramente asincrona
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_tokio_connection().await?;
    
    // pasa la conexion a modo pub/sub
    let mut pubsub = con.into_pubsub();
    pubsub.subscribe("odds_updates").await?;
    
    tracing::info!("Escuchando actualizaciones de cuotas en el canal 'odds_updates'");

    let mut stream = pubsub.into_on_message();
    
    // consume mensajes indefinidamente
    while let Some(msg) = stream.next().await {
        // intentar leer el payload en crudo
        match msg.get_payload::<String>() {
            Ok(payload_str) => {
                // se parsea el json
                match serde_json::from_str::<OddsUpdatePayload>(&payload_str) {
                    Ok(payload) => {
                        if let Ok(parsed_uuid) = uuid::Uuid::parse_str(&payload.match_id) {
                            tracing::debug!("Pub/sub recibido: Match {} Nuevas cuotas {}", payload.match_id, payload.new_odds);
                            
                            // se envia al manager de websockets
                            manager.broadcast_odds_update(MatchId(parsed_uuid), &payload.new_odds);
                        } else {
                            tracing::warn!("Mensaje pub/sub ignorado: match_id inválido '{}'", payload.match_id);
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Mensaje pub/sub ilegible (no es json válido): {} - {}", payload_str, e);
                    }
                }
            },
            Err(e) => {
                tracing::error!("Error al extraer payload del mensaje pub/sub: {}", e);
            }
        }
    }
    Ok(())
}