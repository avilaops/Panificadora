use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, error};

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<TextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<TemplateMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<MediaMessage>,
}

#[derive(Debug, Serialize)]
struct TextMessage {
    preview_url: bool,
    body: String,
}

#[derive(Debug, Serialize)]
struct TemplateMessage {
    name: String,
    language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<TemplateComponent>>,
}

#[derive(Debug, Serialize)]
struct Language {
    code: String,
}

#[derive(Debug, Serialize)]
struct TemplateComponent {
    #[serde(rename = "type")]
    component_type: String,
    parameters: Vec<TemplateParameter>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum TemplateParameter {
    Text { 
        #[serde(rename = "type")]
        param_type: String,
        text: String,
    },
    Image {
        #[serde(rename = "type")]
        param_type: String,
        image: MediaPayload,
    },
}

#[derive(Debug, Serialize)]
struct MediaMessage {
    link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
}

#[derive(Debug, Serialize)]
struct MediaPayload {
    link: String,
}

#[derive(Debug, Deserialize)]
struct SendMessageResponse {
    messages: Vec<MessageStatus>,
}

#[derive(Debug, Deserialize)]
struct MessageStatus {
    id: String,
}

pub struct WhatsAppClient {
    client: Client,
    api_url: String,
    phone_id: String,
    access_token: String,
}

impl WhatsAppClient {
    pub fn new(
        api_url: String,
        phone_id: String,
        access_token: String,
    ) -> Self {
        Self {
            client: Client::new(),
            api_url,
            phone_id,
            access_token,
        }
    }
    
    /// Envia uma mensagem de texto simples
    pub async fn send_text_message(&self, to: &str, message: &str) -> Result<String> {
        info!("Sending WhatsApp text message to {}", to);
        
        let request = SendMessageRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "text".to_string(),
            text: Some(TextMessage {
                preview_url: true,
                body: message.to_string(),
            }),
            template: None,
            image: None,
        };
        
        let url = format!("{}/{}/messages", self.api_url, self.phone_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("WhatsApp API error {}: {}", status, error_text);
            anyhow::bail!("Failed to send WhatsApp message: {}", error_text);
        }
        
        let result: SendMessageResponse = response.json().await?;
        let message_id = result.messages.first()
            .map(|m| m.id.clone())
            .ok_or_else(|| anyhow::anyhow!("No message ID returned"))?;
        
        info!("WhatsApp message sent successfully: {}", message_id);
        Ok(message_id)
    }
    
    /// Envia mensagem usando template pré-aprovado
    pub async fn send_template_message(
        &self,
        to: &str,
        template_name: &str,
        parameters: Vec<String>,
    ) -> Result<String> {
        info!("Sending WhatsApp template '{}' to {}", template_name, to);
        
        let template_params: Vec<TemplateParameter> = parameters
            .into_iter()
            .map(|text| TemplateParameter::Text {
                param_type: "text".to_string(),
                text,
            })
            .collect();
        
        let request = SendMessageRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "template".to_string(),
            text: None,
            template: Some(TemplateMessage {
                name: template_name.to_string(),
                language: Language {
                    code: "pt_BR".to_string(),
                },
                components: Some(vec![TemplateComponent {
                    component_type: "body".to_string(),
                    parameters: template_params,
                }]),
            }),
            image: None,
        };
        
        let url = format!("{}/{}/messages", self.api_url, self.phone_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("WhatsApp template error {}: {}", status, error_text);
            anyhow::bail!("Failed to send template: {}", error_text);
        }
        
        let result: SendMessageResponse = response.json().await?;
        let message_id = result.messages.first()
            .map(|m| m.id.clone())
            .ok_or_else(|| anyhow::anyhow!("No message ID returned"))?;
        
        info!("WhatsApp template sent successfully: {}", message_id);
        Ok(message_id)
    }
    
    /// Envia imagem com caption opcional
    pub async fn send_image(&self, to: &str, image_url: &str, caption: Option<String>) -> Result<String> {
        info!("Sending WhatsApp image to {}", to);
        
        let request = SendMessageRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "image".to_string(),
            text: None,
            template: None,
            image: Some(MediaMessage {
                link: image_url.to_string(),
                caption,
            }),
        };
        
        let url = format!("{}/{}/messages", self.api_url, self.phone_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("WhatsApp image error {}: {}", status, error_text);
            anyhow::bail!("Failed to send image: {}", error_text);
        }
        
        let result: SendMessageResponse = response.json().await?;
        let message_id = result.messages.first()
            .map(|m| m.id.clone())
            .ok_or_else(|| anyhow::anyhow!("No message ID returned"))?;
        
        info!("WhatsApp image sent successfully: {}", message_id);
        Ok(message_id)
    }
    
    /// Marca mensagem como lida
    pub async fn mark_as_read(&self, message_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct MarkReadRequest {
            messaging_product: String,
            status: String,
            message_id: String,
        }
        
        let request = MarkReadRequest {
            messaging_product: "whatsapp".to_string(),
            status: "read".to_string(),
            message_id: message_id.to_string(),
        };
        
        let url = format!("{}/{}/messages", self.api_url, self.phone_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to mark message as read: {}", response.status());
            anyhow::bail!("Failed to mark message as read");
        }
        
        Ok(())
    }
}
