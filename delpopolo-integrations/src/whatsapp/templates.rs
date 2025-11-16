/// Templates pré-aprovados do WhatsApp Business
/// Estes templates devem ser criados e aprovados no Meta Business Manager

pub struct WhatsAppTemplates;

impl WhatsAppTemplates {
    /// Template: Pão quentinho saindo do forno
    pub const FRESH_BREAD: &'static str = "pao_quentinho";
    
    /// Parâmetros: {{1}} = nome do produto
    pub fn fresh_bread_params(product_name: &str) -> Vec<String> {
        vec![product_name.to_string()]
    }
    
    /// Template: Confirmação de pedido
    pub const ORDER_CONFIRMATION: &'static str = "confirmacao_pedido";
    
    /// Parâmetros: {{1}} = número do pedido, {{2}} = valor total
    pub fn order_confirmation_params(order_number: &str, total: &str) -> Vec<String> {
        vec![order_number.to_string(), total.to_string()]
    }
    
    /// Template: Pedido pronto para retirada
    pub const ORDER_READY: &'static str = "pedido_pronto";
    
    /// Parâmetros: {{1}} = número do pedido
    pub fn order_ready_params(order_number: &str) -> Vec<String> {
        vec![order_number.to_string()]
    }
    
    /// Template: Pedido saiu para entrega
    pub const ORDER_DISPATCHED: &'static str = "pedido_enviado";
    
    /// Parâmetros: {{1}} = número do pedido, {{2}} = tempo estimado
    pub fn order_dispatched_params(order_number: &str, eta: &str) -> Vec<String> {
        vec![order_number.to_string(), eta.to_string()]
    }
    
    /// Template: Promoção especial
    pub const PROMOTION: &'static str = "promocao_especial";
    
    /// Parâmetros: {{1}} = descrição da promoção, {{2}} = desconto
    pub fn promotion_params(description: &str, discount: &str) -> Vec<String> {
        vec![description.to_string(), discount.to_string()]
    }
    
    /// Template: Lembrete de carrinho abandonado
    pub const CART_REMINDER: &'static str = "carrinho_abandonado";
    
    /// Parâmetros: {{1}} = nome do cliente
    pub fn cart_reminder_params(customer_name: &str) -> Vec<String> {
        vec![customer_name.to_string()]
    }
    
    /// Template: Agradecimento pós-compra
    pub const THANK_YOU: &'static str = "agradecimento";
    
    /// Parâmetros: {{1}} = nome do cliente, {{2}} = pontos de fidelidade
    pub fn thank_you_params(customer_name: &str, points: &str) -> Vec<String> {
        vec![customer_name.to_string(), points.to_string()]
    }
    
    /// Template: Alerta de estoque baixo (para fornecedores)
    pub const LOW_STOCK_ALERT: &'static str = "estoque_baixo";
    
    /// Parâmetros: {{1}} = nome do produto, {{2}} = quantidade atual
    pub fn low_stock_alert_params(product_name: &str, quantity: &str) -> Vec<String> {
        vec![product_name.to_string(), quantity.to_string()]
    }
    
    /// Template: Dia da pizza / Dia do pão
    pub const SPECIAL_DAY: &'static str = "dia_especial";
    
    /// Parâmetros: {{1}} = nome do evento, {{2}} = desconto
    pub fn special_day_params(event_name: &str, discount: &str) -> Vec<String> {
        vec![event_name.to_string(), discount.to_string()]
    }
    
    /// Template: Aniversário do cliente
    pub const BIRTHDAY: &'static str = "feliz_aniversario";
    
    /// Parâmetros: {{1}} = nome do cliente, {{2}} = código do cupom
    pub fn birthday_params(customer_name: &str, coupon_code: &str) -> Vec<String> {
        vec![customer_name.to_string(), coupon_code.to_string()]
    }
    
    /// Template: Solicitação de feedback
    pub const FEEDBACK_REQUEST: &'static str = "solicitar_avaliacao";
    
    /// Parâmetros: {{1}} = número do pedido
    pub fn feedback_request_params(order_number: &str) -> Vec<String> {
        vec![order_number.to_string()]
    }
    
    /// Template: Boas-vindas novo cliente
    pub const WELCOME: &'static str = "boas_vindas";
    
    /// Parâmetros: {{1}} = nome do cliente
    pub fn welcome_params(customer_name: &str) -> Vec<String> {
        vec![customer_name.to_string()]
    }
    
    /// Template: Reativação de cliente inativo
    pub const REACTIVATION: &'static str = "reativacao_cliente";
    
    /// Parâmetros: {{1}} = nome do cliente, {{2}} = desconto especial
    pub fn reactivation_params(customer_name: &str, discount: &str) -> Vec<String> {
        vec![customer_name.to_string(), discount.to_string()]
    }
}

/// Helper para construir mensagens interativas
pub struct InteractiveMessageBuilder {
    body: String,
    buttons: Vec<(String, String)>, // (id, title)
}

impl InteractiveMessageBuilder {
    pub fn new(body: String) -> Self {
        Self {
            body,
            buttons: Vec::new(),
        }
    }
    
    pub fn add_button(mut self, id: String, title: String) -> Self {
        if self.buttons.len() < 3 {
            self.buttons.push((id, title));
        }
        self
    }
    
    pub fn build(self) -> serde_json::Value {
        serde_json::json!({
            "type": "button",
            "body": {
                "text": self.body
            },
            "action": {
                "buttons": self.buttons.iter().map(|(id, title)| {
                    serde_json::json!({
                        "type": "reply",
                        "reply": {
                            "id": id,
                            "title": title
                        }
                    })
                }).collect::<Vec<_>>()
            }
        })
    }
}

/// Helper para construir mensagens com lista
pub struct ListMessageBuilder {
    body: String,
    button_text: String,
    sections: Vec<ListSection>,
}

pub struct ListSection {
    title: String,
    rows: Vec<ListRow>,
}

pub struct ListRow {
    id: String,
    title: String,
    description: Option<String>,
}

impl ListMessageBuilder {
    pub fn new(body: String, button_text: String) -> Self {
        Self {
            body,
            button_text,
            sections: Vec::new(),
        }
    }
    
    pub fn add_section(mut self, title: String, rows: Vec<ListRow>) -> Self {
        self.sections.push(ListSection { title, rows });
        self
    }
    
    pub fn build(self) -> serde_json::Value {
        serde_json::json!({
            "type": "list",
            "body": {
                "text": self.body
            },
            "action": {
                "button": self.button_text,
                "sections": self.sections.iter().map(|section| {
                    serde_json::json!({
                        "title": section.title,
                        "rows": section.rows.iter().map(|row| {
                            let mut obj = serde_json::json!({
                                "id": row.id,
                                "title": row.title
                            });
                            if let Some(desc) = &row.description {
                                obj["description"] = serde_json::json!(desc);
                            }
                            obj
                        }).collect::<Vec<_>>()
                    })
                }).collect::<Vec<_>>()
            }
        })
    }
}

impl ListRow {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
