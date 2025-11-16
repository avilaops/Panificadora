use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;
use crate::enums::{OrderStatus, OrderSource, PaymentMethod};
use crate::value_objects::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: f64,
    pub unit_price: Money,
    pub total_price: Money,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    
    pub customer_id: Option<Uuid>,
    pub customer_name: Option<String>,
    pub customer_cpf: Option<String>,
    
    pub items: Vec<OrderItem>,
    
    pub subtotal: Money,
    pub discount: Money,
    pub delivery_fee: Money,
    pub total: Money,
    
    pub status: OrderStatus,
    pub source: OrderSource,
    
    pub payment_method: Option<PaymentMethod>,
    pub payment_id: Option<Uuid>,
    pub is_paid: bool,
    
    // Para delivery
    pub delivery_address: Option<String>,
    pub delivery_time: Option<DateTime<Utc>>,
    
    // Para iFood
    pub ifood_order_id: Option<String>,
    pub ifood_reference: Option<String>,
    
    // Para comanda
    pub table_number: Option<String>,
    pub turnstile_entry_id: Option<Uuid>,
    
    pub notes: Option<String>,
    
    pub estimated_preparation_time: Option<i32>,
    pub preparation_started_at: Option<DateTime<Utc>>,
    pub ready_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(source: OrderSource) -> Self {
        let now = Utc::now();
        let order_number = Self::generate_order_number();
        
        Self {
            id: Uuid::new_v4(),
            order_number,
            customer_id: None,
            customer_name: None,
            customer_cpf: None,
            items: vec![],
            subtotal: Money::brl(0.0),
            discount: Money::brl(0.0),
            delivery_fee: Money::brl(0.0),
            total: Money::brl(0.0),
            status: OrderStatus::Pending,
            source,
            payment_method: None,
            payment_id: None,
            is_paid: false,
            delivery_address: None,
            delivery_time: None,
            ifood_order_id: None,
            ifood_reference: None,
            table_number: None,
            turnstile_entry_id: None,
            notes: None,
            estimated_preparation_time: None,
            preparation_started_at: None,
            ready_at: None,
            delivered_at: None,
            cancelled_at: None,
            cancellation_reason: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_item(&mut self, item: OrderItem) {
        self.items.push(item);
        self.recalculate_totals();
    }
    
    pub fn recalculate_totals(&mut self) {
        self.subtotal = self.items.iter()
            .map(|item| item.total_price.clone())
            .fold(Money::brl(0.0), |acc, price| Money::brl(acc.amount + price.amount));
        
        self.total = Money::brl(
            self.subtotal.amount - self.discount.amount + self.delivery_fee.amount
        );
        
        self.updated_at = Utc::now();
    }
    
    pub fn start_preparation(&mut self) {
        self.status = OrderStatus::Preparing;
        self.preparation_started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn mark_ready(&mut self) {
        self.status = OrderStatus::Ready;
        self.ready_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn complete(&mut self) {
        self.status = OrderStatus::Completed;
        self.delivered_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn cancel(&mut self, reason: String) {
        self.status = OrderStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        self.cancellation_reason = Some(reason);
        self.updated_at = Utc::now();
    }
    
    fn generate_order_number() -> String {
        let now = Utc::now();
        format!("ORD-{}{}", 
            now.format("%Y%m%d%H%M%S"),
            &Uuid::new_v4().to_string()[..4].to_uppercase()
        )
    }
}

impl Entity for Order {
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
