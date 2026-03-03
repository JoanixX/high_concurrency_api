// Adaptador primario http para el handler de apuestas
// no hay lógica de negocio, solo traduce http a caso de uso y viceversa

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::application::PlaceBetUseCase;
use crate::domain::{Bet, BetId, MatchId, UserId, Money, Odds};
use super::dto::{ValidateBetRequest, PlaceBetResponse};

#[tracing::instrument(
    name = "Validando una nueva apuesta",
    skip(item, use_case),
    fields(
        user_id = %item.user_id,
        match_id = %item.match_id
    )
)]
pub async fn validate_bet(
    item: web::Json<ValidateBetRequest>,
    use_case: web::Data<PlaceBetUseCase>,
) -> HttpResponse {
    // traducir dto primitivo a una entidad de dominio rica
    let bet_id = BetId::from(Uuid::new_v4());
    let user_id = UserId::from(item.user_id);
    let match_id = MatchId::from(item.match_id);
    
    // Convertir de dto a tipos internos de dominio
    let amount = Money::from_decimal(item.amount);
    let odds = Odds::from_decimal(item.odds);

    let bet = Bet::new(
        bet_id,
        user_id,
        match_id,
        amount,
        odds,
    );

    // Se manda al caso de uso
    match use_case.execute(bet).await {
        Ok(result) => {
            // se traduce la entidad rica a un dto simple
            HttpResponse::Created().json(PlaceBetResponse {
                bet_id: result.bet.id.0,
                user_id: result.bet.user_id.0,
                match_id: result.bet.match_id.0,
                amount: result.bet.amount.to_decimal(),
                odds: result.bet.locked_odds.to_decimal(),
                status: result.bet.status.as_str().to_string(),
            })
        }
        Err(e) => crate::errors::domain_error_to_response(e),
    }
}