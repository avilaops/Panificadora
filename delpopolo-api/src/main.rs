mod routes;
mod state;
mod avila_logger;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;
use state::AppState;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("?? DelPopolo Panificadora API starting...");
    
    let config = delpopolo_infrastructure::Config::load()
        .expect("Failed to load configuration");
    
    // ? Autenticação centralizada com api.avila.inc
    tracing::info!("?? Authenticating with Avila Framework at api.avila.inc...");
    let avila_token = authenticate_with_avila(&config).await?;
    tracing::info!("? Authenticated with Avila Framework");
    
    tracing::info!("?? Connecting to SQLite database...");
    let database = delpopolo_infrastructure::Database::new(&config.database.url)
        .await
        .expect("Failed to connect to database");
    
    tracing::info!("?? Running migrations...");
    database.run_migrations()
        .await
        .expect("Failed to run migrations");
    
    let app_state = web::Data::new(AppState::new(database, config.clone(), avila_token));
    
    let host = config.app.host.clone();
    let port = config.app.port;
    
    tracing::info!("?? Server starting at {}:{}", host, port);
    tracing::info!("?? Domain: {}", config.app.domain);
    tracing::info!("?? Avila API: api.avila.inc");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(routes::configure)
    })
    .bind((host, port))?
    .run()
    .await?;
    
    Ok(())
}

async fn authenticate_with_avila(config: &delpopolo_infrastructure::Config) -> Result<String> {
    use reqwest::Client;
    use serde_json::json;
    
    let client = Client::new();
    
    let response = client
        .post("https://api.avila.inc/auth/token")
        .json(&json!({
            "client_id": "delpopolo-panificadora",
            "client_secret": std::env::var("AVILA_CLIENT_SECRET").unwrap_or_default(),
            "grant_type": "client_credentials"
        }))
        .send()
        .await?;
    
    if !response.status().is_success() {
        tracing::warn!("??  Avila authentication failed - running standalone");
        return Ok("standalone-mode".to_string());
    }
    
    let token_data: serde_json::Value = response.json().await?;
    let token = token_data["access_token"]
        .as_str()
        .unwrap_or("standalone-mode")
        .to_string();
    
    Ok(token)
}
