use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if !email.contains('@') || !email.contains('.') {
            return Err("Email inválido".to_string());
        }
        
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err("Email inválido".to_string());
        }
        
        Ok(Self(email.to_lowercase()))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ValueObject for Email {}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
