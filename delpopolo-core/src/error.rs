use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl CoreError {
    pub fn internal<T: ToString>(msg: T) -> Self {
        CoreError::Internal(msg.to_string())
    }
    
    pub fn not_found<T: ToString>(msg: T) -> Self {
        CoreError::NotFound(msg.to_string())
    }
    
    pub fn validation<T: ToString>(msg: T) -> Self {
        CoreError::Validation(msg.to_string())
    }
    
    pub fn unauthorized<T: ToString>(msg: T) -> Self {
        CoreError::Unauthorized(msg.to_string())
    }
    
    pub fn forbidden<T: ToString>(msg: T) -> Self {
        CoreError::Forbidden(msg.to_string())
    }
    
    pub fn conflict<T: ToString>(msg: T) -> Self {
        CoreError::Conflict(msg.to_string())
    }
    
    pub fn external_service<T: ToString>(msg: T) -> Self {
        CoreError::ExternalService(msg.to_string())
    }
    
    pub fn database<T: ToString>(msg: T) -> Self {
        CoreError::Database(msg.to_string())
    }
    
    pub fn serialization<T: ToString>(msg: T) -> Self {
        CoreError::Serialization(msg.to_string())
    }
}
