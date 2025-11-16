use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use delpopolo_core::traits::Entity;
use crate::enums::{CampaignType, CampaignStatus, CampaignChannel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignRule {
    pub min_purchase_amount: Option<f64>,
    pub applicable_products: Option<Vec<Uuid>>,
    pub applicable_categories: Option<Vec<String>>,
    pub discount_percentage: Option<f32>,
    pub discount_amount: Option<f64>,
    pub free_product_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Campaign {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    
    pub campaign_type: CampaignType,
    pub status: CampaignStatus,
    pub channels: Vec<CampaignChannel>,
    
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    
    // Regras da campanha
    pub rules: Option<CampaignRule>,
    
    // Segmentação de público
    pub target_all_customers: bool,
    pub target_customer_ids: Option<Vec<Uuid>>,
    pub target_vip_only: bool,
    pub target_new_customers: bool,
    
    // Conteúdo da mensagem
    pub message_template: String,
    pub image_url: Option<String>,
    pub cta_text: Option<String>,
    pub cta_url: Option<String>,
    
    // Métricas
    pub total_sent: i32,
    pub total_delivered: i32,
    pub total_opened: i32,
    pub total_clicked: i32,
    pub total_conversions: i32,
    pub revenue_generated: f64,
    
    // Agendamento
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>, // cron expression
    pub next_execution: Option<DateTime<Utc>>,
    
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Campaign {
    pub fn new(
        name: String,
        campaign_type: CampaignType,
        channels: Vec<CampaignChannel>,
        start_date: DateTime<Utc>,
        message_template: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            campaign_type,
            status: CampaignStatus::Draft,
            channels,
            start_date,
            end_date: None,
            rules: None,
            target_all_customers: true,
            target_customer_ids: None,
            target_vip_only: false,
            target_new_customers: false,
            message_template,
            image_url: None,
            cta_text: None,
            cta_url: None,
            total_sent: 0,
            total_delivered: 0,
            total_opened: 0,
            total_clicked: 0,
            total_conversions: 0,
            revenue_generated: 0.0,
            is_recurring: false,
            recurrence_pattern: None,
            next_execution: None,
            created_by: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn activate(&mut self) {
        self.status = CampaignStatus::Active;
        self.updated_at = Utc::now();
    }
    
    pub fn pause(&mut self) {
        self.status = CampaignStatus::Paused;
        self.updated_at = Utc::now();
    }
    
    pub fn complete(&mut self) {
        self.status = CampaignStatus::Completed;
        self.updated_at = Utc::now();
    }
    
    pub fn is_active(&self) -> bool {
        self.status == CampaignStatus::Active && 
        self.start_date <= Utc::now() &&
        self.end_date.map_or(true, |end| end > Utc::now())
    }
    
    pub fn engagement_rate(&self) -> f32 {
        if self.total_sent == 0 {
            return 0.0;
        }
        (self.total_opened as f32 / self.total_sent as f32) * 100.0
    }
    
    pub fn conversion_rate(&self) -> f32 {
        if self.total_sent == 0 {
            return 0.0;
        }
        (self.total_conversions as f32 / self.total_sent as f32) * 100.0
    }
}

impl Entity for Campaign {
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
