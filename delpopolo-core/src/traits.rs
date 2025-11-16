use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository<T> {
    async fn find_by_id(&self, id: Uuid) -> Option<T>;
    async fn find_all(&self) -> Vec<T>;
    async fn save(&self, entity: &T) -> Result<T, Box<dyn std::error::Error>>;
    async fn update(&self, entity: &T) -> Result<T, Box<dyn std::error::Error>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Service {
    type Input;
    type Output;
    
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>>;
}

pub trait Entity {
    fn id(&self) -> Uuid;
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait ValueObject: Clone + PartialEq {}
