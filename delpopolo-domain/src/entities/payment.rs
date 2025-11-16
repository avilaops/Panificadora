use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;
use crate::enums::{PaymentMethod, PaymentStatus};
use crate::value_objects::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    
    pub amount: Money,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    
    // Para cartões
    pub card_last_digits: Option<String>,
    pub card_brand: Option<String>,
    pub authorization_code: Option<String>,
    pub nsu: Option<String>,
    
    // Para PIX
    pub pix_key: Option<String>,
    pub pix_qr_code: Option<String>,
    pub pix_txid: Option<String>,
    
    // Para integrações com POS (Stone, Cielo, etc)
    pub pos_transaction_id: Option<String>,
    pub pos_terminal_id: Option<String>,
    
    // Dinheiro
    pub cash_received: Option<f64>,
    pub change_amount: Option<f64>,
    
    pub notes: Option<String>,
    
    pub paid_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Payment {
    pub fn new(order_id: Uuid, amount: Money, payment_method: PaymentMethod) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            order_id,
            customer_id: None,
            amount,
            payment_method,
            status: PaymentStatus::Pending,
            card_last_digits: None,
            card_brand: None,
            authorization_code: None,
            nsu: None,
            pix_key: None,
            pix_qr_code: None,
            pix_txid: None,
            pos_transaction_id: None,
            pos_terminal_id: None,
            cash_received: None,
            change_amount: None,
            notes: None,
            paid_at: None,
            cancelled_at: None,
            refunded_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn process(&mut self) {
        self.status = PaymentStatus::Processing;
        self.updated_at = Utc::now();
    }
    
    pub fn approve(&mut self) {
        self.status = PaymentStatus::Approved;
        self.paid_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn reject(&mut self) {
        self.status = PaymentStatus::Rejected;
        self.updated_at = Utc::now();
    }
    
    pub fn cancel(&mut self) {
        self.status = PaymentStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn refund(&mut self) {
        self.status = PaymentStatus::Refunded;
        self.refunded_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn is_successful(&self) -> bool {
        self.status == PaymentStatus::Approved
    }
}

impl Entity for Payment {
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
