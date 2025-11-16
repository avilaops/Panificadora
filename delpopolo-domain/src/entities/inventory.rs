use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;
use crate::enums::MovementType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: f64,
    pub reserved_quantity: f64, // Quantidade reservada em pedidos pendentes
    pub available_quantity: f64, // quantity - reserved_quantity
    pub last_movement_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inventory {
    pub fn new(product_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            product_id,
            quantity: 0.0,
            reserved_quantity: 0.0,
            available_quantity: 0.0,
            last_movement_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_quantity(&mut self, quantity: f64) {
        self.quantity += quantity;
        self.recalculate_available();
        self.last_movement_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn remove_quantity(&mut self, quantity: f64) -> Result<(), String> {
        if self.available_quantity < quantity {
            return Err("Insufficient available quantity".to_string());
        }
        self.quantity -= quantity;
        self.recalculate_available();
        self.last_movement_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn reserve(&mut self, quantity: f64) -> Result<(), String> {
        if self.available_quantity < quantity {
            return Err("Insufficient available quantity to reserve".to_string());
        }
        self.reserved_quantity += quantity;
        self.recalculate_available();
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn release_reservation(&mut self, quantity: f64) {
        self.reserved_quantity = (self.reserved_quantity - quantity).max(0.0);
        self.recalculate_available();
        self.updated_at = Utc::now();
    }
    
    fn recalculate_available(&mut self) {
        self.available_quantity = self.quantity - self.reserved_quantity;
    }
}

impl Entity for Inventory {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub product_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: f64,
    pub unit_cost: Option<f64>,
    pub total_cost: Option<f64>,
    
    // Referências
    pub order_id: Option<Uuid>,
    pub supplier_id: Option<Uuid>,
    pub nfe_key: Option<String>, // Chave da NFe quando for entrada por nota fiscal
    
    pub notes: Option<String>,
    pub performed_by: Option<Uuid>, // User ID
    
    pub created_at: DateTime<Utc>,
}

impl InventoryMovement {
    pub fn new(
        product_id: Uuid,
        movement_type: MovementType,
        quantity: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            product_id,
            movement_type,
            quantity,
            unit_cost: None,
            total_cost: None,
            order_id: None,
            supplier_id: None,
            nfe_key: None,
            notes: None,
            performed_by: None,
            created_at: Utc::now(),
        }
    }
    
    pub fn with_cost(mut self, unit_cost: f64) -> Self {
        self.unit_cost = Some(unit_cost);
        self.total_cost = Some(unit_cost * self.quantity);
        self
    }
    
    pub fn with_nfe(mut self, nfe_key: String) -> Self {
        self.nfe_key = Some(nfe_key);
        self
    }
}
