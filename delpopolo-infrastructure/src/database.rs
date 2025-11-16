use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use tracing::{info, error};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to SQLite database: {}", database_url);
        
        let pool = SqlitePool::connect(database_url).await?;
        
        info!("Successfully connected to SQLite database");
        
        Ok(Self { pool })
    }
    
    pub async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        
        info!("Migrations completed successfully");
        Ok(())
    }
    
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                anyhow::anyhow!("Database unhealthy")
            })
    }
}
