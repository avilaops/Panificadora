use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ifood")
            .route(web::post().to(ifood_webhook))
    )
    .service(
        web::resource("/whatsapp")
            .route(web::get().to(whatsapp_verify))
            .route(web::post().to(whatsapp_webhook))
    );
}

async fn ifood_webhook(body: web::Json<serde_json::Value>) -> HttpResponse {
    tracing::info!("Received iFood webhook: {:?}", body);
    // TODO: Processar webhook do iFood
    HttpResponse::Ok().json(json!({"status": "received"}))
}

async fn whatsapp_verify(req: HttpRequest) -> HttpResponse {
    let query = web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string());
    
    if let Ok(params) = query {
        if let (Some(mode), Some(token), Some(challenge)) = 
            (params.get("hub.mode"), params.get("hub.verify_token"), params.get("hub.challenge")) 
        {
            if mode == "subscribe" && token == "your-webhook-verify-token" {
                return HttpResponse::Ok().body(challenge.clone());
            }
        }
    }
    
    HttpResponse::Forbidden().finish()
}

async fn whatsapp_webhook(body: web::Json<serde_json::Value>) -> HttpResponse {
    tracing::info!("Received WhatsApp webhook: {:?}", body);
    // TODO: Processar webhook do WhatsApp
    HttpResponse::Ok().json(json!({"status": "received"}))
}
