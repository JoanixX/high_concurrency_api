// Colocar apuesta
// orquesta la lógica de negocio usando solo los puertos

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::{BetTicket, BetStatus, DomainError};
use crate::domain::ports::{BetRepository, CachePort};

pub struct PlaceBetUseCase {
    bet_repo: Arc<dyn BetRepository>,
    cache: Arc<dyn CachePort>,
}

// respuesta del caso de uso (no es un DTO HTTP)
#[derive(Debug)]
pub struct PlaceBetResult {
    pub bet_id: Uuid,
    pub ticket: BetTicket,
    pub status: BetStatus,
}

impl PlaceBetUseCase {
    pub fn new(
        bet_repo: Arc<dyn BetRepository>,
        cache: Arc<dyn CachePort>,
    ) -> Self {
        Self { bet_repo, cache }
    }

    pub async fn execute(&self, ticket: BetTicket) -> Result<PlaceBetResult, DomainError> {
        // Validaciones de dominio
        if ticket.amount <= 0.0 {
            return Err(DomainError::Validation(
                "el monto debe ser mayor a 0".to_string(),
            ));
        }
        if ticket.odds <= 1.0 {
            return Err(DomainError::Validation(
                "las odds deben ser mayores a 1.0".to_string(),
            ));
        }

        let bet_id = Uuid::new_v4();
        let status = BetStatus::Validated;

        // persistir via puerto
        self.bet_repo.save(bet_id, &ticket, &status).await?;

        tracing::info!(
            bet_id = %bet_id,
            user_id = %ticket.user_id,
            "apuesta validada y persistida"
        );

        // Cache de última apuesta (best-effort, no falla el caso de uso)
        let cache_key = format!("last_bet:{}", ticket.user_id);
        if let Err(e) = self.cache.set(&cache_key, &bet_id.to_string(), 60).await {
            tracing::warn!("no se pudo actualizar la cache: {:?}", e);
        }

        Ok(PlaceBetResult {
            bet_id,
            ticket,
            status,
        })
    }
}