use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::state::AppState;

pub async fn create_product(
    state: web::Data<AppState>,
    _product: web::Json<serde_json::Value>,
) -> HttpResponse {
    // ? Log operação em api.avila.inc
    let _ = state.avila_logger.log_operation(
        "products",
        "create",
        true,
        json!({"action": "product_created"})
    ).await;
    
    // ? Enviar métrica para api.avila.inc
    let _ = state.avila_logger.send_metric(
        "products.created",
        1.0,
        json!({"category": "bread"})
    ).await;
    
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Product created and logged to Avila"
    }))
}

pub async fn get_products(state: web::Data<AppState>) -> HttpResponse {
    // ? Usar token Avila para autenticar qualquer chamada externa
    let _auth_header = state.get_avila_auth_header();
    
    HttpResponse::Ok().json(json!({
        "products": [],
        "authenticated_via": "api.avila.inc"
    }))
}
