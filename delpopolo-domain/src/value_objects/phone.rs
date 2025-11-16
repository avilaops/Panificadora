use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phone(String);

impl Phone {
    pub fn new(phone: String) -> Result<Self, String> {
        let cleaned = phone.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if cleaned.len() < 10 || cleaned.len() > 11 {
            return Err("Telefone deve ter 10 ou 11 d�gitos".to_string());
        }
        
        Ok(Self(cleaned))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn formatted(&self) -> String {
        if self.0.len() == 11 {
            format!(
                "({}) {}-{}",
                &self.0[0..2],
                &self.0[2..7],
                &self.0[7..11]
            )
        } else {
            format!(
                "({}) {}-{}",
                &self.0[0..2],
                &self.0[2..6],
                &self.0[6..10]
            )
        }
    }
    
    pub fn whatsapp_format(&self) -> String {
        format!("+55{}", self.0)
    }
}

impl ValueObject for Phone {}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
}
