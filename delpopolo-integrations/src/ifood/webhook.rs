use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code")]
pub enum IFoodWebhookEvent {
    #[serde(rename = "PLC")]
    OrderPlaced { order_id: String },
    
    #[serde(rename = "CFM")]
    OrderConfirmed { order_id: String },
    
    #[serde(rename = "DSP")]
    OrderDispatched { order_id: String },
    
    #[serde(rename = "RTP")]
    OrderReadyToPickup { order_id: String },
    
    #[serde(rename = "CON")]
    OrderConcluded { order_id: String },
    
    #[serde(rename = "CAN")]
    OrderCancelled { 
        order_id: String,
        cancellation_code: Option<String>,
        cancellation_reason: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFoodWebhook {
    pub code: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "fullCode")]
    pub full_code: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl IFoodWebhook {
    pub fn parse_event(&self) -> Option<IFoodWebhookEvent> {
        match self.code.as_str() {
            "PLC" => Some(IFoodWebhookEvent::OrderPlaced {
                order_id: self.order_id.clone(),
            }),
            "CFM" => Some(IFoodWebhookEvent::OrderConfirmed {
                order_id: self.order_id.clone(),
            }),
            "DSP" => Some(IFoodWebhookEvent::OrderDispatched {
                order_id: self.order_id.clone(),
            }),
            "RTP" => Some(IFoodWebhookEvent::OrderReadyToPickup {
                order_id: self.order_id.clone(),
            }),
            "CON" => Some(IFoodWebhookEvent::OrderConcluded {
                order_id: self.order_id.clone(),
            }),
            "CAN" => Some(IFoodWebhookEvent::OrderCancelled {
                order_id: self.order_id.clone(),
                cancellation_code: None,
                cancellation_reason: None,
            }),
            _ => None,
        }
    }
}
