// Adaptador primario http para los handlers de autenticación
// estos adaptadores traducen http a un caso de uso y devuelven un http response

use actix_web::{web, HttpResponse};
use crate::application::{RegisterUserUseCase, LoginUserUseCase};
use super::dto::{CreateUserRequest, LoginRequest};

#[tracing::instrument(name = "Registrando nuevo usuario", skip(form, use_case))]
pub async fn register(
    form: web::Json<CreateUserRequest>,
    use_case: web::Data<RegisterUserUseCase>,
) -> HttpResponse {
    match use_case.execute(&form.email, &form.password, &form.name).await {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Created",
            "user_id": result.user_id
        })),
        Err(e) => crate::errors::domain_error_to_response(e),
    }
}

#[tracing::instrument(name = "Login de usuario", skip(form, use_case))]
pub async fn login(
    form: web::Json<LoginRequest>,
    use_case: web::Data<LoginUserUseCase>,
) -> HttpResponse {
    match use_case.execute(&form.email, &form.password).await {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Authenticated",
            "user_id": result.user_id,
            "name": result.name
        })),
        Err(e) => crate::errors::domain_error_to_response(e),
    }
}