use redis::{Client, aio::ConnectionManager, AsyncCommands};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Cache {
    manager: ConnectionManager,
}

impl Cache {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }
    
    pub async fn get<T: for<'de> Deserialize<'de>>(&mut self, key: &str) -> Result<Option<T>> {
        let value: Option<String> = self.manager.get(key).await?;
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }
    
    pub async fn set<T: Serialize>(&mut self, key: &str, value: &T, ttl_seconds: Option<usize>) -> Result<()> {
        let serialized = serde_json::to_string(value)?;
        if let Some(ttl) = ttl_seconds {
            self.manager.set_ex(key, serialized, ttl).await?;
        } else {
            self.manager.set(key, serialized).await?;
        }
        Ok(())
    }
    
    pub async fn delete(&mut self, key: &str) -> Result<()> {
        self.manager.del(key).await?;
        Ok(())
    }
    
    pub async fn exists(&mut self, key: &str) -> Result<bool> {
        let exists: bool = self.manager.exists(key).await?;
        Ok(exists)
    }
    
    pub async fn increment(&mut self, key: &str) -> Result<i64> {
        let value: i64 = self.manager.incr(key, 1).await?;
        Ok(value)
    }
    
    pub async fn expire(&mut self, key: &str, seconds: usize) -> Result<()> {
        self.manager.expire(key, seconds as i64).await?;
        Ok(())
    }
    
    pub async fn health_check(&mut self) -> Result<()> {
        let _: String = redis::cmd("PING").query_async(&mut self.manager).await?;
        Ok(())
    }
}
