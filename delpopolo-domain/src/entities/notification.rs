use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use delpopolo_core::traits::Entity;
use crate::enums::{NotificationType, NotificationChannel, NotificationStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub recipient_id: Option<Uuid>,
    pub recipient_email: Option<String>,
    pub recipient_phone: Option<String>,
    pub recipient_fcm_token: Option<String>,
    
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub status: NotificationStatus,
    
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    
    pub retry_count: i32,
    pub max_retries: i32,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        notification_type: NotificationType,
        channel: NotificationChannel,
        title: String,
        message: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            recipient_id: None,
            recipient_email: None,
            recipient_phone: None,
            recipient_fcm_token: None,
            notification_type,
            channel,
            status: NotificationStatus::Pending,
            title,
            message,
            data: None,
            sent_at: None,
            delivered_at: None,
            read_at: None,
            failed_at: None,
            failure_reason: None,
            retry_count: 0,
            max_retries: 3,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn mark_sent(&mut self) {
        self.status = NotificationStatus::Sent;
        self.sent_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn mark_delivered(&mut self) {
        self.status = NotificationStatus::Delivered;
        self.delivered_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn mark_read(&mut self) {
        self.status = NotificationStatus::Read;
        self.read_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn mark_failed(&mut self, reason: String) {
        self.status = NotificationStatus::Failed;
        self.failed_at = Some(Utc::now());
        self.failure_reason = Some(reason);
        self.retry_count += 1;
        self.updated_at = Utc::now();
    }
    
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}

impl Entity for Notification {
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
