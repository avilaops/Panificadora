use delpopolo_infrastructure::{Database, Config};
use crate::avila_logger::AvilaLogger;

pub struct AppState {
    pub database: Database,
    pub config: Config,
    pub avila_token: String,
    pub avila_logger: AvilaLogger,
}

impl AppState {
    pub fn new(database: Database, config: Config, avila_token: String) -> Self {
        let avila_logger = AvilaLogger::new(avila_token.clone());
        
        Self {
            database,
            config,
            avila_token,
            avila_logger,
        }
    }
    
    pub fn get_avila_auth_header(&self) -> String {
        format!("Bearer {}", self.avila_token)
    }
}
