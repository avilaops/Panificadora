use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    // Saudações
    Greeting,
    Goodbye,
    
    // Pedidos
    PlaceOrder,
    CheckOrderStatus,
    CancelOrder,
    ModifyOrder,
    
    // Cardápio e Produtos
    ViewMenu,
    ProductInfo,
    ProductAvailability,
    ProductPrice,
    Recommendations,
    
    // Horários e Localização
    BusinessHours,
    Location,
    DeliveryArea,
    
    // Promoções
    CurrentPromotions,
    SpecialOffers,
    
    // Suporte
    HumanAgent,
    Complaint,
    Compliment,
    
    // Outros
    Unknown,
}

pub struct IntentClassifier {
    patterns: Vec<(regex::Regex, Intent)>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = Vec::new();
        
        // Saudações
        patterns.push((
            regex::Regex::new(r"(?i)(oi|olá|ola|bom dia|boa tarde|boa noite|hey|e aí)").unwrap(),
            Intent::Greeting,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(tchau|até logo|falou|bye|adeus)").unwrap(),
            Intent::Goodbye,
        ));
        
        // Pedidos
        patterns.push((
            regex::Regex::new(r"(?i)(quero|gostaria de|fazer|criar|novo).*(pedido|pedir|comprar)").unwrap(),
            Intent::PlaceOrder,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(onde está|status|acompanhar|ver).*(meu pedido|pedido)").unwrap(),
            Intent::CheckOrderStatus,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(cancelar|desistir).*(pedido)").unwrap(),
            Intent::CancelOrder,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(alterar|modificar|mudar|trocar).*(pedido)").unwrap(),
            Intent::ModifyOrder,
        ));
        
        // Cardápio
        patterns.push((
            regex::Regex::new(r"(?i)(cardápio|menu|o que tem|quais|produtos|tem o que)").unwrap(),
            Intent::ViewMenu,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(tem|existe|vende).*(pão|bolo|biscoito|salgado|doce)").unwrap(),
            Intent::ProductAvailability,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(quanto custa|preço|valor).*(pão|bolo|biscoito|produto)").unwrap(),
            Intent::ProductPrice,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(recomendar|sugerir|indicar|o que é bom)").unwrap(),
            Intent::Recommendations,
        ));
        
        // Horário e Local
        patterns.push((
            regex::Regex::new(r"(?i)(horário|hora|abre|fecha|funciona|aberto)").unwrap(),
            Intent::BusinessHours,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(endereço|onde fica|localização|como chegar)").unwrap(),
            Intent::Location,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(entrega|delivery|área|região|bairro|entregar)").unwrap(),
            Intent::DeliveryArea,
        ));
        
        // Promoções
        patterns.push((
            regex::Regex::new(r"(?i)(promoção|oferta|desconto|barato)").unwrap(),
            Intent::CurrentPromotions,
        ));
        
        // Suporte
        patterns.push((
            regex::Regex::new(r"(?i)(falar com|atendente|humano|pessoa|alguém)").unwrap(),
            Intent::HumanAgent,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(reclamação|reclamar|problema|ruim|péssimo)").unwrap(),
            Intent::Complaint,
        ));
        
        patterns.push((
            regex::Regex::new(r"(?i)(parabéns|obrigado|excelente|ótimo|maravilhoso)").unwrap(),
            Intent::Compliment,
        ));
        
        Self { patterns }
    }
    
    pub fn classify(&self, message: &str) -> Intent {
        for (pattern, intent) in &self.patterns {
            if pattern.is_match(message) {
                return intent.clone();
            }
        }
        Intent::Unknown
    }
    
    pub fn extract_product_name(&self, message: &str) -> Option<String> {
        let products = vec![
            "pão francês", "pão de forma", "pão integral", "pão de queijo",
            "baguete", "croissant", "brioche", "ciabatta",
            "bolo de chocolate", "bolo de cenoura", "bolo de fubá",
            "sonho", "bomba", "carolina", "éclair",
            "coxinha", "pastel", "empada", "esfiha",
        ];
        
        let message_lower = message.to_lowercase();
        for product in products {
            if message_lower.contains(product) {
                return Some(product.to_string());
            }
        }
        
        None
    }
    
    pub fn extract_order_number(&self, message: &str) -> Option<String> {
        let re = regex::Regex::new(r"(?i)(pedido|número|#)\s*([A-Z0-9-]+)").unwrap();
        re.captures(message)
            .and_then(|caps| caps.get(2))
            .map(|m| m.as_str().to_string())
    }
}

impl Default for IntentClassifier {
    fn default() -> Self {
        Self::new()
    }
}
