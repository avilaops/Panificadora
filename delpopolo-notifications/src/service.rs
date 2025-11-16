use anyhow::Result;
use tracing::info;
use delpopolo_domain::{Notification, NotificationType, NotificationChannel};

use super::email::EmailService;
use super::push::PushNotificationService;
use super::sms::SMSService;

pub struct NotificationService {
    email_service: EmailService,
    push_service: PushNotificationService,
    sms_service: SMSService,
}

impl NotificationService {
    pub fn new(
        email_service: EmailService,
        push_service: PushNotificationService,
        sms_service: SMSService,
    ) -> Self {
        Self {
            email_service,
            push_service,
            sms_service,
        }
    }
    
    pub async fn send(&self, notification: &Notification) -> Result<()> {
        info!("Sending notification {} via {:?}", notification.id, notification.channel);
        
        match notification.channel {
            NotificationChannel::Email => {
                if let Some(email) = &notification.recipient_email {
                    self.email_service
                        .send_email(
                            email,
                            &notification.title,
                            &notification.message,
                            None,
                        )
                        .await?;
                }
            }
            NotificationChannel::Push => {
                if let Some(fcm_token) = &notification.recipient_fcm_token {
                    self.push_service
                        .send_notification(
                            fcm_token,
                            &notification.title,
                            &notification.message,
                            notification.data.clone(),
                        )
                        .await?;
                }
            }
            NotificationChannel::SMS => {
                if let Some(phone) = &notification.recipient_phone {
                    self.sms_service
                        .send_sms(phone, &notification.message)
                        .await?;
                }
            }
            NotificationChannel::WhatsApp => {
                // Será implementado via WhatsApp client
                info!("WhatsApp notification will be sent via integration module");
            }
        }
        
        Ok(())
    }
    
    pub async fn send_batch(&self, notifications: Vec<&Notification>) -> Result<Vec<uuid::Uuid>> {
        let mut sent_ids = Vec::new();
        
        for notification in notifications {
            match self.send(notification).await {
                Ok(_) => sent_ids.push(notification.id),
                Err(e) => {
                    tracing::error!("Failed to send notification {}: {}", notification.id, e);
                }
            }
        }
        
        Ok(sent_ids)
    }
}
