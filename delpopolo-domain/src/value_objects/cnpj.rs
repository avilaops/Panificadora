use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cnpj(String);

impl Cnpj {
    pub fn new(cnpj: String) -> Result<Self, String> {
        let cleaned = cnpj.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if cleaned.len() != 14 {
            return Err("CNPJ deve ter 14 dígitos".to_string());
        }
        
        if !Self::validate(&cleaned) {
            return Err("CNPJ inválido".to_string());
        }
        
        Ok(Self(cleaned))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn formatted(&self) -> String {
        format!(
            "{}.{}.{}/{}-{}",
            &self.0[0..2],
            &self.0[2..5],
            &self.0[5..8],
            &self.0[8..12],
            &self.0[12..14]
        )
    }
    
    fn validate(cnpj: &str) -> bool {
        if cnpj.chars().all(|c| c == cnpj.chars().next().unwrap()) {
            return false;
        }
        
        let digits: Vec<u32> = cnpj.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        let first_digit = Self::calculate_digit(&digits[0..12], &[5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
        let second_digit = Self::calculate_digit(&digits[0..13], &[6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
        
        digits[12] == first_digit && digits[13] == second_digit
    }
    
    fn calculate_digit(digits: &[u32], weights: &[u32]) -> u32 {
        let sum: u32 = digits.iter().zip(weights.iter()).map(|(d, w)| d * w).sum();
        let remainder = sum % 11;
        if remainder < 2 {
            0
        } else {
            11 - remainder
        }
    }
}

impl ValueObject for Cnpj {}

impl std::fmt::Display for Cnpj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
}
