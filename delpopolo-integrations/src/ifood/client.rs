use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, error};

use super::models::*;

pub struct IFoodClient {
    client: Client,
    api_url: String,
    client_id: String,
    client_secret: String,
    merchant_id: String,
    access_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    #[serde(rename = "grantType")]
    grant_type: String,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: i64,
}

impl IFoodClient {
    pub fn new(
        api_url: String,
        client_id: String,
        client_secret: String,
        merchant_id: String,
    ) -> Self {
        Self {
            client: Client::new(),
            api_url,
            client_id,
            client_secret,
            merchant_id,
            access_token: None,
        }
    }
    
    pub async fn authenticate(&mut self) -> Result<()> {
        info!("Authenticating with iFood API");
        
        let token_request = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
        };
        
        let response = self.client
            .post(format!("{}/oauth/token", self.api_url))
            .json(&token_request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to authenticate with iFood: {}", response.status());
            anyhow::bail!("iFood authentication failed");
        }
        
        let token_response: TokenResponse = response.json().await?;
        self.access_token = Some(token_response.access_token);
        
        info!("Successfully authenticated with iFood");
        Ok(())
    }
    
    pub async fn get_orders(&self, status: Option<OrderStatus>) -> Result<Vec<IFoodOrder>> {
        self.ensure_authenticated()?;
        
        let mut url = format!("{}/order/v1.0/orders", self.api_url);
        if let Some(s) = status {
            url = format!("{}?status={}", url, s.as_str());
        }
        
        let response = self.client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to get orders from iFood: {}", response.status());
            anyhow::bail!("Failed to get orders");
        }
        
        let orders: Vec<IFoodOrder> = response.json().await?;
        Ok(orders)
    }
    
    pub async fn get_order(&self, order_id: &str) -> Result<IFoodOrder> {
        self.ensure_authenticated()?;
        
        let url = format!("{}/order/v1.0/orders/{}", self.api_url, order_id);
        
        let response = self.client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to get order {} from iFood: {}", order_id, response.status());
            anyhow::bail!("Failed to get order");
        }
        
        let order: IFoodOrder = response.json().await?;
        Ok(order)
    }
    
    pub async fn confirm_order(&self, order_id: &str) -> Result<()> {
        self.ensure_authenticated()?;
        
        let url = format!("{}/order/v1.0/orders/{}/confirm", self.api_url, order_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to confirm order {}: {}", order_id, response.status());
            anyhow::bail!("Failed to confirm order");
        }
        
        info!("Order {} confirmed successfully", order_id);
        Ok(())
    }
    
    pub async fn start_preparation(&self, order_id: &str) -> Result<()> {
        self.ensure_authenticated()?;
        
        let url = format!("{}/order/v1.0/orders/{}/startPreparation", self.api_url, order_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to start preparation for order {}: {}", order_id, response.status());
            anyhow::bail!("Failed to start preparation");
        }
        
        info!("Preparation started for order {}", order_id);
        Ok(())
    }
    
    pub async fn mark_ready_for_pickup(&self, order_id: &str) -> Result<()> {
        self.ensure_authenticated()?;
        
        let url = format!("{}/order/v1.0/orders/{}/readyForPickup", self.api_url, order_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to mark order {} ready for pickup: {}", order_id, response.status());
            anyhow::bail!("Failed to mark ready for pickup");
        }
        
        info!("Order {} marked as ready for pickup", order_id);
        Ok(())
    }
    
    pub async fn dispatch_order(&self, order_id: &str) -> Result<()> {
        self.ensure_authenticated()?;
        
        let url = format!("{}/order/v1.0/orders/{}/dispatch", self.api_url, order_id);
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to dispatch order {}: {}", order_id, response.status());
            anyhow::bail!("Failed to dispatch order");
        }
        
        info!("Order {} dispatched successfully", order_id);
        Ok(())
    }
    
    pub async fn cancel_order(&self, order_id: &str, cancellation_code: &str, reason: &str) -> Result<()> {
        self.ensure_authenticated()?;
        
        #[derive(Serialize)]
        struct CancelRequest {
            #[serde(rename = "cancellationCode")]
            cancellation_code: String,
            reason: String,
        }
        
        let url = format!("{}/order/v1.0/orders/{}/cancel", self.api_url, order_id);
        
        let cancel_request = CancelRequest {
            cancellation_code: cancellation_code.to_string(),
            reason: reason.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .json(&cancel_request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            error!("Failed to cancel order {}: {}", order_id, response.status());
            anyhow::bail!("Failed to cancel order");
        }
        
        info!("Order {} cancelled successfully", order_id);
        Ok(())
    }
    
    fn ensure_authenticated(&self) -> Result<()> {
        if self.access_token.is_none() {
            anyhow::bail!("Not authenticated. Call authenticate() first");
        }
        Ok(())
    }
}
