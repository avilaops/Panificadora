use reqwest::Client;

pub struct WhatsAppClient {
    _client: Client,
    _api_url: String,
    _phone_id: String,
    _access_token: String,
}

impl WhatsAppClient {
    pub fn new(
        api_url: String,
        phone_id: String,
        access_token: String,
    ) -> Self {
        Self {
            _client: Client::new(),
            _api_url: api_url,
            _phone_id: phone_id,
            _access_token: access_token,
        }
    }
    
    // TODO: Implementar métodos
}
