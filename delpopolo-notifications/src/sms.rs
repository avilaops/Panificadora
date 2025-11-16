use anyhow::Result;

pub struct SMSService {
    // Integração com provedor de SMS (Twilio, AWS SNS, etc)
}

impl SMSService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn send_sms(&self, _to: &str, _message: &str) -> Result<()> {
        // TODO: Implementar integração com provedor de SMS
        Ok(())
    }
}

impl Default for SMSService {
    fn default() -> Self {
        Self::new()
    }
}
