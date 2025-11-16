use crate::intent::{Intent, IntentClassifier};

pub struct NLPEngine {
    classifier: IntentClassifier,
}

impl NLPEngine {
    pub fn new() -> Self {
        Self {
            classifier: IntentClassifier::new(),
        }
    }
    
    pub fn process(&self, message: &str) -> NLPResult {
        let normalized = self.normalize_text(message);
        let intent = self.classifier.classify(&normalized);
        let confidence = self.calculate_confidence(&normalized, &intent);
        
        let entities = self.extract_entities(&normalized, &intent);
        
        NLPResult {
            intent,
            confidence,
            entities,
            original_message: message.to_string(),
            normalized_message: normalized,
        }
    }
    
    fn normalize_text(&self, text: &str) -> String {
        text.trim()
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'á' | 'à' | 'â' | 'ã' => 'a',
                'é' | 'è' | 'ê' => 'e',
                'í' | 'ì' | 'î' => 'i',
                'ó' | 'ò' | 'ô' | 'õ' => 'o',
                'ú' | 'ù' | 'û' => 'u',
                'ç' => 'c',
                _ => c,
            })
            .collect()
    }
    
    fn calculate_confidence(&self, _message: &str, intent: &Intent) -> f32 {
        match intent {
            Intent::Unknown => 0.1,
            _ => 0.85,
        }
    }
    
    fn extract_entities(&self, message: &str, intent: &Intent) -> Vec<Entity> {
        let mut entities = Vec::new();
        
        match intent {
            Intent::ProductInfo | Intent::ProductAvailability | Intent::ProductPrice => {
                if let Some(product) = self.classifier.extract_product_name(message) {
                    entities.push(Entity {
                        entity_type: EntityType::Product,
                        value: product,
                        confidence: 0.9,
                    });
                }
            }
            Intent::CheckOrderStatus | Intent::CancelOrder | Intent::ModifyOrder => {
                if let Some(order_number) = self.classifier.extract_order_number(message) {
                    entities.push(Entity {
                        entity_type: EntityType::OrderNumber,
                        value: order_number,
                        confidence: 0.95,
                    });
                }
            }
            _ => {}
        }
        
        entities
    }
}

impl Default for NLPEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct NLPResult {
    pub intent: Intent,
    pub confidence: f32,
    pub entities: Vec<Entity>,
    pub original_message: String,
    pub normalized_message: String,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Product,
    OrderNumber,
    Quantity,
    Time,
    Date,
    Location,
    Price,
}
