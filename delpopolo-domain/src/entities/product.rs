use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use delpopolo_core::traits::Entity;
use crate::enums::{ProductCategory, UnitOfMeasure};
use crate::value_objects::Money;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    
    pub sku: String,
    pub barcode: Option<String>,
    pub category: ProductCategory,
    pub unit_of_measure: UnitOfMeasure,
    
    pub price: Money,
    pub cost: Money,
    
    pub stock_quantity: f64,
    pub min_stock_level: f64,
    pub max_stock_level: Option<f64>,
    
    pub is_active: bool,
    pub is_available_online: bool,
    
    pub image_url: Option<String>,
    pub weight: Option<f64>,
    pub preparation_time_minutes: Option<i32>,
    
    pub supplier_id: Option<Uuid>,
    
    // Dados da NFe quando produto é cadastrado via nota fiscal
    pub nfe_ncm: Option<String>, // Código NCM
    pub nfe_cest: Option<String>, // Código CEST
    pub nfe_cfop: Option<String>, // Código CFOP
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        category: ProductCategory,
        unit_of_measure: UnitOfMeasure,
        price: Money,
        cost: Money,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            sku: Self::generate_sku(),
            barcode: None,
            category,
            unit_of_measure,
            price,
            cost,
            stock_quantity: 0.0,
            min_stock_level: 10.0,
            max_stock_level: None,
            is_active: true,
            is_available_online: true,
            image_url: None,
            weight: None,
            preparation_time_minutes: None,
            supplier_id: None,
            nfe_ncm: None,
            nfe_cest: None,
            nfe_cfop: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn is_low_stock(&self) -> bool {
        self.stock_quantity <= self.min_stock_level
    }
    
    pub fn is_out_of_stock(&self) -> bool {
        self.stock_quantity <= 0.0
    }
    
    pub fn can_fulfill(&self, quantity: f64) -> bool {
        self.stock_quantity >= quantity
    }
    
    fn generate_sku() -> String {
        format!("PRD-{}", Uuid::new_v4().to_string()[..8].to_uppercase())
    }
}

impl Entity for Product {
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
