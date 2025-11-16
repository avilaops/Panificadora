use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Placed,
    Confirmed,
    Integrated,
    ReadyToPickup,
    Dispatched,
    Concluded,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Placed => "PLACED",
            OrderStatus::Confirmed => "CONFIRMED",
            OrderStatus::Integrated => "INTEGRATED",
            OrderStatus::ReadyToPickup => "READY_TO_PICKUP",
            OrderStatus::Dispatched => "DISPATCHED",
            OrderStatus::Concluded => "CONCLUDED",
            OrderStatus::Cancelled => "CANCELLED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Delivery,
    Takeout,
    Indoor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodOrder {
    pub id: String,
    #[serde(rename = "displayId")]
    pub display_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "orderTiming")]
    pub order_timing: String,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    pub customer: IFoodCustomer,
    pub items: Vec<IFoodItem>,
    pub total: IFoodTotal,
    pub payments: IFoodPayments,
    pub delivery: Option<IFoodDelivery>,
    #[serde(rename = "preparationStartDateTime")]
    pub preparation_start_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodCustomer {
    pub id: String,
    pub name: String,
    pub phone: String,
    #[serde(rename = "documentNumber")]
    pub document_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodItem {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    #[serde(rename = "unitPrice")]
    pub unit_price: f64,
    #[serde(rename = "totalPrice")]
    pub total_price: f64,
    #[serde(rename = "externalCode")]
    pub external_code: Option<String>,
    pub observations: Option<String>,
    #[serde(rename = "subItems")]
    pub sub_items: Option<Vec<IFoodSubItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodSubItem {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    #[serde(rename = "unitPrice")]
    pub unit_price: f64,
    #[serde(rename = "totalPrice")]
    pub total_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodTotal {
    #[serde(rename = "subTotal")]
    pub sub_total: f64,
    #[serde(rename = "deliveryFee")]
    pub delivery_fee: f64,
    pub benefits: f64,
    #[serde(rename = "orderAmount")]
    pub order_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodPayments {
    pub methods: Vec<IFoodPaymentMethod>,
    pub pending: f64,
    pub prepaid: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodPaymentMethod {
    pub method: String,
    pub value: f64,
    pub currency: String,
    #[serde(rename = "type")]
    pub payment_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodDelivery {
    pub mode: String,
    #[serde(rename = "deliveryDateTime")]
    pub delivery_date_time: Option<DateTime<Utc>>,
    #[serde(rename = "deliveryAddress")]
    pub delivery_address: IFoodAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodAddress {
    #[serde(rename = "streetName")]
    pub street_name: String,
    #[serde(rename = "streetNumber")]
    pub street_number: String,
    pub complement: Option<String>,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub country: String,
    pub reference: Option<String>,
}
