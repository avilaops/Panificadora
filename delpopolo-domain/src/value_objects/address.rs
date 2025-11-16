use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
}

impl Address {
    pub fn new(
        street: String,
        number: String,
        neighborhood: String,
        city: String,
        state: String,
        zip_code: String,
    ) -> Self {
        Self {
            street,
            number,
            complement: None,
            neighborhood,
            city,
            state,
            zip_code,
            country: "Brasil".to_string(),
        }
    }
    
    pub fn with_complement(mut self, complement: String) -> Self {
        self.complement = Some(complement);
        self
    }
    
    pub fn formatted(&self) -> String {
        let complement_str = self.complement
            .as_ref()
            .map(|c| format!(", {}", c))
            .unwrap_or_default();
        
        format!(
            "{}, {}{} - {} - {}/{} - CEP: {}",
            self.street,
            self.number,
            complement_str,
            self.neighborhood,
            self.city,
            self.state,
            self.zip_code
        )
    }
}

impl ValueObject for Address {}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
}
