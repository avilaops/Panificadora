use reqwest::Client;
use serde_json::json;
use anyhow::Result;

pub struct AvilaLogger {
    client: Client,
    token: String,
}

impl AvilaLogger {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }
    
    /// Log todas as operações em api.avila.inc/logs
    pub async fn log_operation(&self, service: &str, operation: &str, success: bool, details: serde_json::Value) -> Result<()> {
        let payload = json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "service": service,
            "operation": operation,
            "success": success,
            "details": details,
            "source": "delpopolo-panificadora"
        });
        
        let _response = self.client
            .post("https://api.avila.inc/v1/logs")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .await;
        
        Ok(())
    }
    
    /// Enviar métricas para api.avila.inc/metrics
    pub async fn send_metric(&self, metric_name: &str, value: f64, tags: serde_json::Value) -> Result<()> {
        let payload = json!({
            "metric": metric_name,
            "value": value,
            "tags": tags,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "source": "delpopolo-panificadora"
        });
        
        let _response = self.client
            .post("https://api.avila.inc/v1/metrics")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .await;
        
        Ok(())
    }
    
    /// Log de eventos de negócio
    pub async fn log_business_event(&self, event_type: &str, data: serde_json::Value) -> Result<()> {
        self.log_operation("business", event_type, true, data).await
    }
}
