use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, error};

#[derive(Debug, Serialize)]
struct FCMMessage {
    to: String,
    notification: FCMNotification,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct FCMNotification {
    title: String,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FCMResponse {
    #[serde(default)]
    success: i32,
    #[serde(default)]
    failure: i32,
}

pub struct PushNotificationService {
    client: Client,
    server_key: String,
    api_url: String,
}

impl PushNotificationService {
    pub fn new(server_key: String, api_url: String) -> Self {
        Self {
            client: Client::new(),
            server_key,
            api_url,
        }
    }
    
    pub async fn send_notification(
        &self,
        fcm_token: &str,
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<()> {
        info!("Sending push notification: {}", title);
        
        let message = FCMMessage {
            to: fcm_token.to_string(),
            notification: FCMNotification {
                title: title.to_string(),
                body: body.to_string(),
                icon: Some("ic_notification".to_string()),
                sound: Some("default".to_string()),
            },
            data,
        };
        
        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("key={}", self.server_key))
            .header("Content-Type", "application/json")
            .json(&message)
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("FCM error: {}", response.status());
            anyhow::bail!("Failed to send push notification");
        }
        
        let result: FCMResponse = response.json().await?;
        
        if result.success > 0 {
            info!("Push notification sent successfully");
            Ok(())
        } else {
            error!("FCM reported failure");
            Err(anyhow::anyhow!("Push notification failed"))
        }
    }
    
    /// Notifica pedido pronto
    pub async fn notify_order_ready(
        &self,
        fcm_token: &str,
        order_number: &str,
    ) -> Result<()> {
        self.send_notification(
            fcm_token,
            "?? Seu pedido está pronto!",
            &format!("O pedido {} já pode ser retirado.", order_number),
            Some(serde_json::json!({
                "type": "order_ready",
                "order_number": order_number
            })),
        ).await
    }
    
    /// Notifica saída para entrega
    pub async fn notify_order_dispatched(
        &self,
        fcm_token: &str,
        order_number: &str,
        eta_minutes: i32,
    ) -> Result<()> {
        self.send_notification(
            fcm_token,
            "?? Pedido saiu para entrega",
            &format!("Seu pedido {} está a caminho! Previsão: {} minutos.", order_number, eta_minutes),
            Some(serde_json::json!({
                "type": "order_dispatched",
                "order_number": order_number,
                "eta": eta_minutes
            })),
        ).await
    }
    
    /// Notifica promoção
    pub async fn notify_promotion(
        &self,
        fcm_token: &str,
        title: &str,
        message: &str,
    ) -> Result<()> {
        self.send_notification(
            fcm_token,
            title,
            message,
            Some(serde_json::json!({
                "type": "promotion"
            })),
        ).await
    }
    
    /// Notifica pão quentinho
    pub async fn notify_fresh_bread(
        &self,
        fcm_token: &str,
        product_name: &str,
    ) -> Result<()> {
        self.send_notification(
            fcm_token,
            "?? Pão quentinho saindo do forno!",
            &format!("{} acabou de sair do forno! Corra antes que acabe!", product_name),
            Some(serde_json::json!({
                "type": "fresh_bread",
                "product": product_name
            })),
        ).await
    }
}
