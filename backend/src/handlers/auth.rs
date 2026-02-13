use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use secrecy::{ExposeSecret, Secret};
use crate::domain::models::{CreateUserRequest, LoginRequest};

#[tracing::instrument(name = "Registrando nuevo usuario", skip(form, pool))]
pub async fn register(
    form: web::Json<CreateUserRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user_id = Uuid::new_v4();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(form.password.as_bytes(), &salt)
        .expect("Falló al hashear la contraseña")
        .to_string();

    match sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, name, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        user_id,
        form.email,
        password_hash,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "created", "user_id": user_id})),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Login de usuario", skip(form, pool))]
pub async fn login(
    form: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user = match sqlx::query!(
        r#"SELECT id, password_hash, name FROM users WHERE email = $1"#,
        form.email
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Credenciales inválidas"),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).expect("Hash inválido en la db");
    if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
         HttpResponse::Ok().json(serde_json::json!({
             "status": "authenticated", 
             "user_id": user.id,
             "name": user.name
         }))
    } else {
        HttpResponse::Unauthorized().body("Credenciales inválidas")
    }
}
