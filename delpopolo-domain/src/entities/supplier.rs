use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use delpopolo_core::traits::Entity;
use crate::value_objects::{Email, Phone, Address, Cnpj};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierProduct {
    pub product_id: Uuid,
    pub supplier_sku: String,
    pub unit_price: f64,
    pub min_order_quantity: Option<f64>,
    pub lead_time_days: Option<i32>,
    pub is_available: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Supplier {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 255))]
    pub trade_name: Option<String>,
    
    pub cnpj: Option<Cnpj>,
    
    #[validate(email)]
    pub email: Option<Email>,
    
    pub phone: Option<Phone>,
    pub whatsapp: Option<Phone>,
    pub address: Option<Address>,
    
    pub contact_person: Option<String>,
    
    pub is_active: bool,
    pub is_preferred: bool,
    
    // Cat�logo de produtos do fornecedor
    pub products: Vec<SupplierProduct>,
    
    // Dados de integra��o via API
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    
    // Dados de integra��o via Excel
    pub excel_catalog_url: Option<String>,
    pub excel_last_sync: Option<DateTime<Utc>>,
    
    // M�tricas de qualidade
    pub rating: Option<f32>,
    pub total_orders: i32,
    pub on_time_delivery_rate: Option<f32>,
    
    pub payment_terms: Option<String>,
    pub delivery_days: Option<Vec<String>>,
    pub min_order_value: Option<f64>,
    
    pub notes: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Supplier {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            trade_name: None,
            cnpj: None,
            email: None,
            phone: None,
            whatsapp: None,
            address: None,
            contact_person: None,
            is_active: true,
            is_preferred: false,
            products: vec![],
            api_url: None,
            api_key: None,
            excel_catalog_url: None,
            excel_last_sync: None,
            rating: None,
            total_orders: 0,
            on_time_delivery_rate: None,
            payment_terms: None,
            delivery_days: None,
            min_order_value: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_product(&mut self, product: SupplierProduct) {
        self.products.push(product);
        self.updated_at = Utc::now();
    }
    
    pub fn update_product_price(&mut self, product_id: Uuid, new_price: f64) {
        if let Some(product) = self.products.iter_mut().find(|p| p.product_id == product_id) {
            product.unit_price = new_price;
            product.last_updated = Utc::now();
        }
        self.updated_at = Utc::now();
    }
    
    pub fn get_best_price_for_product(&self, product_id: Uuid) -> Option<f64> {
        self.products
            .iter()
            .find(|p| p.product_id == product_id && p.is_available)
            .map(|p| p.unit_price)
    }
}

impl Entity for Supplier {
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
