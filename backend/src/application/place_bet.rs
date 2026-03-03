// Colocar apuesta
// orquesta la lógica de negocio usando solo los puertos

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::{
    Bet, BetId, MatchId, UserId, DomainError,
    BetValidationPolicy, StandardBetValidationPolicy,
};
use crate::domain::ports::{BetRepository, MatchRepository, UserRepository, CachePort};

pub struct PlaceBetUseCase {
    bet_repo: Arc<dyn BetRepository>,
    match_repo: Arc<dyn MatchRepository>,
    user_repo: Arc<dyn UserRepository>,
    cache: Arc<dyn CachePort>,
    policy: StandardBetValidationPolicy,
}

// respuesta del caso de uso
#[derive(Debug)]
pub struct PlaceBetResult {
    pub bet: Bet,
}

impl PlaceBetUseCase {
    pub fn new(
        bet_repo: Arc<dyn BetRepository>,
        match_repo: Arc<dyn MatchRepository>,
        user_repo: Arc<dyn UserRepository>,
        cache: Arc<dyn CachePort>,
    ) -> Self {
        Self { 
            bet_repo, 
            match_repo,
            user_repo,
            cache,
            policy: StandardBetValidationPolicy::new(),
        }
    }

    pub async fn execute(&self, mut bet: Bet) -> Result<PlaceBetResult, DomainError> {
        // 1. obtener la informacion actual del partido para validar cuotas y estado
        let sport_match = self.match_repo.find_by_id(bet.match_id).await?
            .ok_or_else(|| DomainError::NotFound)?; // el partido no existe

        // 2. obtener el balance actual del usuario
        let user_balance = self.user_repo.get_balance(bet.user_id).await?;

        // 3. ejecutar las reglas de negocio (politica de validacion)
        self.policy.validate(&bet, &sport_match, &user_balance)?;

        // 4. si la validacion pasa, aceptar la apuesta
        bet.accept();

        // 5. persistir vía puerto
        self.bet_repo.save(&bet).await?;

        tracing::info!(
            bet_id = %bet.id,
            user_id = %bet.user_id,
            "apuesta validada y persistida"
        );

        // 6. cache de ultima apuesta (best-effort)
        let cache_key = format!("last_bet:{}", bet.user_id);
        if let Err(e) = self.cache.set(&cache_key, &bet.id.to_string(), 60).await {
            tracing::warn!("no se pudo actualizar la cache: {:?}", e);
        }

        Ok(PlaceBetResult {
            bet,
        })
    }
}