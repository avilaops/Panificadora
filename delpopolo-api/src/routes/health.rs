use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(health_check))
    )
    .service(
        web::resource("/ready")
            .route(web::get().to(readiness_check))
    );
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "delpopolo-api",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn readiness_check() -> HttpResponse {
    // TODO: Check database, redis, rabbitmq connections
    HttpResponse::Ok().json(json!({
        "status": "ready"
    }))
}
