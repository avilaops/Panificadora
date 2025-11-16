use reqwest::Client;

pub struct StoneClient {
    _client: Client,
    _api_url: String,
    _api_key: String,
}

impl StoneClient {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            _client: Client::new(),
            _api_url: api_url,
            _api_key: api_key,
        }
    }
    
    // TODO: Implementar métodos de pagamento
}
