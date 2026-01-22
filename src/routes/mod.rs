use actix_web::web;
use crate::handlers::health_check;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check));
}
