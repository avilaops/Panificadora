use uuid::Uuid;
use serde::{Deserialize, Serialize};
use delpopolo_domain::{Product, Supplier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQuote {
    pub supplier_id: Uuid,
    pub supplier_name: String,
    pub unit_price: f64,
    pub min_order_quantity: f64,
    pub lead_time_days: i32,
    pub total_cost: f64,
    pub is_preferred: bool,
    pub rating: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplenishmentSuggestion {
    pub product_id: Uuid,
    pub product_name: String,
    pub current_stock: f64,
    pub min_stock_level: f64,
    pub suggested_order_quantity: f64,
    pub quotes: Vec<SupplierQuote>,
    pub best_quote: Option<SupplierQuote>,
    pub urgency_score: f32,
}

impl ReplenishmentSuggestion {
    pub fn new(product: &Product, current_stock: f64) -> Self {
        let suggested_quantity = Self::calculate_order_quantity(
            current_stock,
            product.min_stock_level,
            product.max_stock_level,
        );
        
        let urgency = Self::calculate_urgency(current_stock, product.min_stock_level);
        
        Self {
            product_id: product.id,
            product_name: product.name.clone(),
            current_stock,
            min_stock_level: product.min_stock_level,
            suggested_order_quantity: suggested_quantity,
            quotes: Vec::new(),
            best_quote: None,
            urgency_score: urgency,
        }
    }
    
    pub fn add_quote(&mut self, quote: SupplierQuote) {
        self.quotes.push(quote);
        self.update_best_quote();
    }
    
    fn update_best_quote(&mut self) {
        if self.quotes.is_empty() {
            self.best_quote = None;
            return;
        }
        
        // Ordenar por: preferido > menor preço > melhor rating
        let mut sorted_quotes = self.quotes.clone();
        sorted_quotes.sort_by(|a, b| {
            // Primeiro: fornecedor preferido
            match (a.is_preferred, b.is_preferred) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
            
            // Segundo: menor preço
            match a.total_cost.partial_cmp(&b.total_cost) {
                Some(std::cmp::Ordering::Equal) => {}
                Some(ordering) => return ordering,
                None => {}
            }
            
            // Terceiro: melhor rating
            match (a.rating, b.rating) {
                (Some(ra), Some(rb)) => rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
        
        self.best_quote = sorted_quotes.first().cloned();
    }
    
    fn calculate_order_quantity(current: f64, min: f64, max: Option<f64>) -> f64 {
        let target = max.unwrap_or(min * 5.0);
        (target - current).max(min * 2.0)
    }
    
    fn calculate_urgency(current: f64, min: f64) -> f32 {
        if current <= 0.0 {
            1.0 // Urgência máxima
        } else if current < min * 0.5 {
            0.9
        } else if current < min {
            0.7
        } else if current < min * 1.5 {
            0.5
        } else {
            0.2
        }
    }
}

pub struct ReplenishmentEngine;

impl ReplenishmentEngine {
    pub fn generate_suggestion(
        product: &Product,
        current_stock: f64,
        suppliers: Vec<&Supplier>,
    ) -> ReplenishmentSuggestion {
        let mut suggestion = ReplenishmentSuggestion::new(product, current_stock);
        
        for supplier in suppliers {
            if let Some(supplier_product) = supplier.products
                .iter()
                .find(|sp| sp.product_id == product.id && sp.is_available)
            {
                let min_qty = supplier_product.min_order_quantity.unwrap_or(1.0);
                let actual_qty = suggestion.suggested_order_quantity.max(min_qty);
                
                let quote = SupplierQuote {
                    supplier_id: supplier.id,
                    supplier_name: supplier.name.clone(),
                    unit_price: supplier_product.unit_price,
                    min_order_quantity: min_qty,
                    lead_time_days: supplier_product.lead_time_days.unwrap_or(3),
                    total_cost: supplier_product.unit_price * actual_qty,
                    is_preferred: supplier.is_preferred,
                    rating: supplier.rating,
                };
                
                suggestion.add_quote(quote);
            }
        }
        
        suggestion
    }
    
    pub fn calculate_reorder_point(
        daily_consumption: f64,
        lead_time_days: i32,
        safety_stock_days: i32,
    ) -> f64 {
        daily_consumption * (lead_time_days + safety_stock_days) as f64
    }
    
    pub fn calculate_economic_order_quantity(
        annual_demand: f64,
        ordering_cost: f64,
        holding_cost_per_unit: f64,
    ) -> f64 {
        ((2.0 * annual_demand * ordering_cost) / holding_cost_per_unit).sqrt()
    }
}
