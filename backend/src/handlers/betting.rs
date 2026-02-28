// Adaptador primario http para el handler de apuestas
// no hay lógica de negocio, solo traduce http a caso de uso y viceversa

use actix_web::{web, HttpResponse};
use crate::application::PlaceBetUseCase;
use crate::domain::BetTicket;
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
    // traducir dto a una entidad de dominio
    let ticket = BetTicket {
        user_id: item.user_id,
        match_id: item.match_id,
        amount: item.amount,
        odds: item.odds,
    };

    // Se manda al caso de uso
    match use_case.execute(ticket).await {
        Ok(result) => {
            // se traduce el resultado y se devuelbe un dto
            HttpResponse::Ok().json(PlaceBetResponse {
                bet_id: result.bet_id,
                user_id: result.ticket.user_id,
                match_id: result.ticket.match_id,
                amount: result.ticket.amount,
                odds: result.ticket.odds,
                status: result.status.as_str().to_string(),
            })
        }
        Err(e) => crate::errors::domain_error_to_response(e),
    }
}