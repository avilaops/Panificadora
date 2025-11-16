use config::{Config as ConfigLoader, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMQConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IFoodConfig {
    pub client_id: String,
    pub client_secret: String,
    pub merchant_id: String,
    pub api_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhatsAppConfig {
    pub phone_id: String,
    pub business_id: String,
    pub access_token: String,
    pub webhook_verify_token: String,
    pub api_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub domain: String,
    pub frontend_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub jwt: JwtConfig,
    pub ifood: IFoodConfig,
    pub whatsapp: WhatsAppConfig,
    pub smtp: SmtpConfig,
    pub app: AppConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config = ConfigLoader::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::default().separator("__"))
            .build()?;
        
        config.try_deserialize()
    }
}
