// Este es el mapeo de errores de dominio a respuestas http
// basicamente un puente entre la arquitectura y el protocolo http

use actix_web::HttpResponse;
use crate::domain::DomainError;

// convierte un error de dominio en httpResponse
pub fn domain_error_to_response(error: DomainError) -> HttpResponse {
    match &error {
        DomainError::Validation(msg) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Error de validación",
                "message": msg
            }))
        }
        DomainError::NotFound => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "No encontrado"
            }))
        }
        DomainError::AuthenticationFailed => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Credenciales inválidas"
            }))
        }
        DomainError::Duplicate(msg) => {
            HttpResponse::Conflict().json(serde_json::json!({
                "error": "Entidad duplicada",
                "message": msg
            }))
        }
        DomainError::Internal(msg) => {
            // logueamos el error real pero no lo exponemos al cliente
            tracing::error!("error interno: {}", msg);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Error interno del servidor"
            }))
        }
    }
}