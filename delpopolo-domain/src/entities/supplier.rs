use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;
use crate::value_objects::{Cnpj, Email, Phone};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: Uuid,
    
    pub name: String,
    
    pub trade_name: Option<String>,
    
    pub cnpj: Option<Cnpj>,
    
    pub email: Option<Email>,
    
    pub phone: Option<Phone>,
    pub whatsapp: Option<String>,
    pub contact_person: Option<String>,
    
    pub is_active: bool,
    pub is_preferred: bool,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity for Supplier {
    fn id(&self) -> Uuid { self.id }
    fn created_at(&self) -> DateTime<Utc> { self.created_at }
    fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
}
