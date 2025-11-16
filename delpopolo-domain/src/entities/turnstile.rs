use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnstileEntry {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_cpf: Option<String>,
    pub customer_name: Option<String>,
    
    pub entry_time: DateTime<Utc>,
    pub exit_time: Option<DateTime<Utc>>,
    
    pub table_number: Option<String>,
    pub order_id: Option<Uuid>,
    
    pub total_spent: f64,
    pub is_paid: bool,
    
    pub notes: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TurnstileEntry {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            customer_id: None,
            customer_cpf: None,
            customer_name: None,
            entry_time: now,
            exit_time: None,
            table_number: None,
            order_id: None,
            total_spent: 0.0,
            is_paid: false,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn register_exit(&mut self) {
        self.exit_time = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn duration_minutes(&self) -> Option<i64> {
        self.exit_time.map(|exit| {
            (exit - self.entry_time).num_minutes()
        })
    }
    
    pub fn is_still_inside(&self) -> bool {
        self.exit_time.is_none()
    }
}

impl Entity for TurnstileEntry {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Default for TurnstileEntry {
    fn default() -> Self {
        Self::new()
    }
}
