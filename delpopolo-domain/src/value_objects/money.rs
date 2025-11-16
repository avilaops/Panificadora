use serde::{Deserialize, Serialize};
use delpopolo_core::traits::ValueObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

impl Money {
    pub fn brl(amount: f64) -> Self {
        Self {
            amount,
            currency: "BRL".to_string(),
        }
    }
    
    pub fn new(amount: f64, currency: String) -> Self {
        Self { amount, currency }
    }
    
    pub fn add(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot add different currencies".to_string());
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }
    
    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot subtract different currencies".to_string());
        }
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }
    
    pub fn multiply(&self, factor: f64) -> Money {
        Money {
            amount: self.amount * factor,
            currency: self.currency.clone(),
        }
    }
    
    pub fn is_positive(&self) -> bool {
        self.amount > 0.0
    }
    
    pub fn is_zero(&self) -> bool {
        self.amount == 0.0
    }
    
    pub fn formatted(&self) -> String {
        format!("R$ {:.2}", self.amount)
    }
}

impl ValueObject for Money {}

impl Default for Money {
    fn default() -> Self {
        Self::brl(0.0)
    }
}
