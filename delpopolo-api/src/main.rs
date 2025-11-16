mod routes;
mod handlers;
mod middleware;
mod dto;
mod state;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
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
    
    let database = delpopolo_infrastructure::Database::new(&config.database.url)
        .await
        .expect("Failed to connect to database");
    
    database.run_migrations()
        .await
        .expect("Failed to run migrations");
    
    let cache = delpopolo_infrastructure::Cache::new(&config.redis.url)
        .await
        .expect("Failed to connect to Redis");
    
    let app_state = web::Data::new(AppState::new(database, cache, config.clone()));
    
    let host = config.app.host.clone();
    let port = config.app.port;
    
    tracing::info!("?? Server starting at {}:{}", host, port);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(routes::configure)
    })
    .bind((host, port))?
    .run()
    .await?;
    
    Ok(())
}
