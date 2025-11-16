use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cpf(String);

impl Cpf {
    pub fn new(cpf: String) -> Result<Self, String> {
        let cleaned = cpf.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if cleaned.len() != 11 {
            return Err("CPF deve ter 11 dígitos".to_string());
        }
        
        if !Self::validate(&cleaned) {
            return Err("CPF inválido".to_string());
        }
        
        Ok(Self(cleaned))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn formatted(&self) -> String {
        format!(
            "{}.{}.{}-{}",
            &self.0[0..3],
            &self.0[3..6],
            &self.0[6..9],
            &self.0[9..11]
        )
    }
    
    fn validate(cpf: &str) -> bool {
        if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
            return false;
        }
        
        let digits: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        let first_digit = Self::calculate_digit(&digits[0..9]);
        let second_digit = Self::calculate_digit(&digits[0..10]);
        
        digits[9] == first_digit && digits[10] == second_digit
    }
    
    fn calculate_digit(digits: &[u32]) -> u32 {
        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d * (digits.len() as u32 + 1 - i as u32))
            .sum();
        
        let remainder = sum % 11;
        if remainder < 2 {
            0
        } else {
            11 - remainder
        }
    }
}

impl ValueObject for Cpf {}

impl std::fmt::Display for Cpf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
}
