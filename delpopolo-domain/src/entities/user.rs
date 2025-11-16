use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use delpopolo_core::traits::Entity;
use crate::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password_hash: String,
    
    pub role: UserRole,
    pub is_active: bool,
    
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password_hash,
            role,
            is_active: true,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_last_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn can_manage_inventory(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Manager | UserRole::InventoryManager)
    }
    
    pub fn can_manage_orders(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Manager | UserRole::Cashier)
    }
}

impl Entity for User {
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
