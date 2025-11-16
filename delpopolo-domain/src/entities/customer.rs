use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use delpopolo_core::traits::Entity;
use crate::value_objects::{Cpf, Email, Phone, Address};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    pub cpf: Cpf,
    
    #[validate(email)]
    pub email: Option<Email>,
    
    pub phone: Option<Phone>,
    pub address: Option<Address>,
    
    pub birth_date: Option<DateTime<Utc>>,
    
    pub is_active: bool,
    pub accepts_marketing: bool,
    
    // Gamifica��o e fidelidade
    pub loyalty_points: i32,
    pub total_orders: i32,
    pub total_spent: f64,
    
    // Prefer�ncias
    pub favorite_products: Vec<Uuid>,
    pub dietary_restrictions: Vec<String>,
    
    // WhatsApp opt-in
    pub whatsapp_optin: bool,
    pub whatsapp_number: Option<String>,
    
    // Notifica��es push
    pub fcm_token: Option<String>,
    
    pub last_order_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Customer {
    pub fn new(name: String, cpf: Cpf) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            cpf,
            email: None,
            phone: None,
            address: None,
            birth_date: None,
            is_active: true,
            accepts_marketing: false,
            loyalty_points: 0,
            total_orders: 0,
            total_spent: 0.0,
            favorite_products: vec![],
            dietary_restrictions: vec![],
            whatsapp_optin: false,
            whatsapp_number: None,
            fcm_token: None,
            last_order_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_loyalty_points(&mut self, points: i32) {
        self.loyalty_points += points;
        self.updated_at = Utc::now();
    }
    
    pub fn is_vip(&self) -> bool {
        self.total_spent > 1000.0 || self.total_orders > 50
    }
    
    pub fn can_receive_whatsapp(&self) -> bool {
        self.whatsapp_optin && self.whatsapp_number.is_some()
    }
}

impl Entity for Customer {
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
