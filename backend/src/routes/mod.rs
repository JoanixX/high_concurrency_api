use actix_web::web;
use crate::handlers::{health_check, validate_bet, register, login, ws_upgrade_handler};
use actix_governor::{Governor, GovernorConfig};
use crate::middlewares::rate_limit::RealIpExtractor;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Endpoints de lectura / sin estado (sin rate limit de mutación)
    cfg.route("/health_check", web::get().to(health_check));
    cfg.route("/ws/{user_id}", web::get().to(ws_upgrade_handler));
}

pub fn configure_rate_limited_routes(cfg: &mut web::ServiceConfig) {
    // Endpoints de mutación (protegidos globalmente por IP real)
    cfg.route("/bets", web::post().to(validate_bet));
    cfg.route("/register", web::post().to(register));
    cfg.route("/login", web::post().to(login));
}