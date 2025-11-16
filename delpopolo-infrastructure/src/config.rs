use config::{Config as ConfigLoader, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub domain: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config = ConfigLoader::builder()
            .add_source(Environment::default().separator("__"))
            .build()?;
        
        config.try_deserialize()
    }
}
