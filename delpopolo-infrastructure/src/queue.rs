use lapin::{
    Connection, ConnectionProperties, Channel,
    options::{QueueDeclareOptions, BasicPublishOptions, BasicConsumeOptions},
    types::FieldTable,
    BasicProperties,
};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Queue {
    channel: Channel,
}

impl Queue {
    pub async fn new(amqp_url: &str) -> Result<Self> {
        let conn = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        Ok(Self { channel })
    }
    
    pub async fn declare_queue(&self, queue_name: &str) -> Result<()> {
        self.channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }
    
    pub async fn publish<T: Serialize>(
        &self,
        queue_name: &str,
        message: &T,
    ) -> Result<()> {
        let payload = serde_json::to_vec(message)?;
        
        self.channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default().with_delivery_mode(2), // persistent
            )
            .await?
            .await?;
        
        Ok(())
    }
    
    pub async fn consume<F, T>(
        &self,
        queue_name: &str,
        consumer_tag: &str,
        mut handler: F,
    ) -> Result<()>
    where
        F: FnMut(T) -> Result<()> + Send + 'static,
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        let mut consumer = self.channel
            .basic_consume(
                queue_name,
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                if let Ok((channel, delivery)) = delivery {
                    if let Ok(message) = serde_json::from_slice::<T>(&delivery.data) {
                        if handler(message).is_ok() {
                            let _ = channel.basic_ack(delivery.delivery_tag, Default::default()).await;
                        } else {
                            let _ = channel.basic_nack(
                                delivery.delivery_tag,
                                Default::default(),
                                Default::default(),
                            ).await;
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
}
