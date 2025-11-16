use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertLevel {
    Critical,  // Estoque zerado
    High,      // Abaixo do mínimo
    Medium,    // Próximo do mínimo
    Low,       // Informativo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAlert {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub alert_level: AlertLevel,
    pub current_quantity: f64,
    pub min_stock_level: f64,
    pub suggested_order_quantity: f64,
    pub best_supplier_id: Option<Uuid>,
    pub best_supplier_name: Option<String>,
    pub best_price: Option<f64>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged: bool,
}

impl StockAlert {
    pub fn new(
        product_id: Uuid,
        product_name: String,
        current_quantity: f64,
        min_stock_level: f64,
    ) -> Self {
        let alert_level = Self::determine_level(current_quantity, min_stock_level);
        let message = Self::generate_message(&product_name, current_quantity, min_stock_level, &alert_level);
        let suggested_order_quantity = Self::calculate_order_quantity(current_quantity, min_stock_level);
        
        Self {
            id: Uuid::new_v4(),
            product_id,
            product_name,
            alert_level,
            current_quantity,
            min_stock_level,
            suggested_order_quantity,
            best_supplier_id: None,
            best_supplier_name: None,
            best_price: None,
            message,
            created_at: Utc::now(),
            acknowledged: false,
        }
    }
    
    fn determine_level(current: f64, min: f64) -> AlertLevel {
        if current <= 0.0 {
            AlertLevel::Critical
        } else if current < min {
            AlertLevel::High
        } else if current <= min * 1.2 {
            AlertLevel::Medium
        } else {
            AlertLevel::Low
        }
    }
    
    fn generate_message(
        product_name: &str,
        current: f64,
        min: f64,
        level: &AlertLevel,
    ) -> String {
        match level {
            AlertLevel::Critical => {
                format!("? CRÍTICO: {} está ZERADO no estoque!", product_name)
            }
            AlertLevel::High => {
                format!(
                    "?? URGENTE: {} está com {} unidades (mínimo: {})",
                    product_name, current, min
                )
            }
            AlertLevel::Medium => {
                format!(
                    "? ATENÇÃO: {} está próximo do mínimo ({}/{})",
                    product_name, current, min
                )
            }
            AlertLevel::Low => {
                format!(
                    "?? INFO: {} está em nível adequado ({})",
                    product_name, current
                )
            }
        }
    }
    
    fn calculate_order_quantity(current: f64, min: f64) -> f64 {
        let target = min * 3.0; // Pedir para 3x o mínimo
        (target - current).max(0.0)
    }
    
    pub fn add_supplier_info(
        &mut self,
        supplier_id: Uuid,
        supplier_name: String,
        price: f64,
    ) {
        self.best_supplier_id = Some(supplier_id);
        self.best_supplier_name = Some(supplier_name.clone());
        self.best_price = Some(price);
        
        self.message.push_str(&format!(
            "\n?? Melhor fornecedor: {} - R$ {:.2}",
            supplier_name, price
        ));
    }
    
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }
}

pub struct AlertManager {
    alerts: Vec<StockAlert>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
        }
    }
    
    pub fn add_alert(&mut self, alert: StockAlert) {
        self.alerts.push(alert);
    }
    
    pub fn get_critical_alerts(&self) -> Vec<&StockAlert> {
        self.alerts
            .iter()
            .filter(|a| matches!(a.alert_level, AlertLevel::Critical) && !a.acknowledged)
            .collect()
    }
    
    pub fn get_high_priority_alerts(&self) -> Vec<&StockAlert> {
        self.alerts
            .iter()
            .filter(|a| matches!(a.alert_level, AlertLevel::High) && !a.acknowledged)
            .collect()
    }
    
    pub fn get_all_unacknowledged(&self) -> Vec<&StockAlert> {
        self.alerts
            .iter()
            .filter(|a| !a.acknowledged)
            .collect()
    }
    
    pub fn acknowledge_alert(&mut self, alert_id: Uuid) {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledge();
        }
    }
    
    pub fn clear_acknowledged(&mut self) {
        self.alerts.retain(|a| !a.acknowledged);
    }
    
    pub fn alert_count(&self) -> usize {
        self.alerts.iter().filter(|a| !a.acknowledged).count()
    }
    
    pub fn critical_count(&self) -> usize {
        self.get_critical_alerts().len()
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}
