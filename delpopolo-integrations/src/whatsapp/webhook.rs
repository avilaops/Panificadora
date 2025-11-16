use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhook {
    pub object: String,
    pub entry: Vec<WebhookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEntry {
    pub id: String,
    pub changes: Vec<WebhookChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookChange {
    pub value: WebhookValue,
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookValue {
    pub messaging_product: String,
    pub metadata: WebhookMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<WhatsAppMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<MessageStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<ButtonContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<InteractiveContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaContent {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonContent {
    pub text: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveContent {
    #[serde(rename = "type")]
    pub interactive_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_reply: Option<ButtonReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_reply: Option<ListReply>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStatus {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<PricingInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<ConversationOrigin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationOrigin {
    #[serde(rename = "type")]
    pub origin_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    pub billable: bool,
    pub pricing_model: String,
    pub category: String,
}

impl WhatsAppWebhook {
    pub fn extract_messages(&self) -> Vec<&WhatsAppMessage> {
        self.entry
            .iter()
            .flat_map(|entry| &entry.changes)
            .filter_map(|change| change.value.messages.as_ref())
            .flatten()
            .collect()
    }
    
    pub fn extract_statuses(&self) -> Vec<&MessageStatus> {
        self.entry
            .iter()
            .flat_map(|entry| &entry.changes)
            .filter_map(|change| change.value.statuses.as_ref())
            .flatten()
            .collect()
    }
}

impl WhatsAppMessage {
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_ref().map(|t| t.body.as_str())
    }
    
    pub fn get_button_payload(&self) -> Option<&str> {
        self.button.as_ref().map(|b| b.payload.as_str())
    }
    
    pub fn is_text(&self) -> bool {
        self.message_type == "text"
    }
    
    pub fn is_button(&self) -> bool {
        self.message_type == "button"
    }
    
    pub fn is_interactive(&self) -> bool {
        self.message_type == "interactive"
    }
}
