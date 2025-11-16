use actix_web::web;

pub mod health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .configure(health::configure)
    );
}
