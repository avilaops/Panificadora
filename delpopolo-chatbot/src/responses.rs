use crate::intent::Intent;
use crate::context::ConversationContext;

pub struct ResponseGenerator;

impl ResponseGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, intent: &Intent, context: &ConversationContext) -> String {
        match intent {
            Intent::Greeting => self.greeting_response(context),
            Intent::Goodbye => self.goodbye_response(),
            Intent::PlaceOrder => self.place_order_response(context),
            Intent::CheckOrderStatus => self.check_order_status_response(),
            Intent::CancelOrder => self.cancel_order_response(),
            Intent::ModifyOrder => self.modify_order_response(),
            Intent::ViewMenu => self.view_menu_response(),
            Intent::ProductInfo => self.product_info_response(),
            Intent::ProductAvailability => self.product_availability_response(),
            Intent::ProductPrice => self.product_price_response(),
            Intent::Recommendations => self.recommendations_response(),
            Intent::BusinessHours => self.business_hours_response(),
            Intent::Location => self.location_response(),
            Intent::DeliveryArea => self.delivery_area_response(),
            Intent::CurrentPromotions => self.current_promotions_response(),
            Intent::SpecialOffers => self.special_offers_response(),
            Intent::HumanAgent => self.human_agent_response(),
            Intent::Complaint => self.complaint_response(),
            Intent::Compliment => self.compliment_response(),
            Intent::Unknown => self.unknown_response(),
        }
    }
    
    fn greeting_response(&self, context: &ConversationContext) -> String {
        if context.message_count == 0 {
            "Olá! Bem-vindo à DelPopolo Panificadora! ??\n\n\
            Como posso ajudar você hoje?\n\n\
            • Ver cardápio ??\n\
            • Fazer pedido ??\n\
            • Acompanhar pedido ??\n\
            • Promoções ??".to_string()
        } else {
            "Oi novamente! Em que posso ajudar?".to_string()
        }
    }
    
    fn goodbye_response(&self) -> String {
        "Até logo! Volte sempre! ??\n\
        Nosso pão quentinho te espera! ??".to_string()
    }
    
    fn place_order_response(&self, _context: &ConversationContext) -> String {
        "Ótimo! Vamos fazer seu pedido! ??\n\n\
        Você pode:\n\
        • Digitar o nome dos produtos que deseja\n\
        • Ver nosso cardápio completo\n\
        • Pedir recomendações\n\n\
        O que você gostaria de pedir?".to_string()
    }
    
    fn check_order_status_response(&self) -> String {
        "Para verificar seu pedido, me informe o número do pedido.\n\
        Exemplo: #ORD-20250116-AB12".to_string()
    }
    
    fn cancel_order_response(&self) -> String {
        "Entendo que deseja cancelar um pedido.\n\
        Por favor, me informe o número do pedido que deseja cancelar.".to_string()
    }
    
    fn modify_order_response(&self) -> String {
        "Para modificar seu pedido, preciso do número do pedido.\n\
        Após isso, me diga o que deseja alterar.".to_string()
    }
    
    fn view_menu_response(&self) -> String {
        "?? **Nosso Cardápio DelPopolo**\n\n\
        ?? **Pães**\n\
        • Pão Francês - R$ 0,80\n\
        • Pão de Forma - R$ 8,50\n\
        • Pão Integral - R$ 9,90\n\
        • Baguete - R$ 12,00\n\n\
        ?? **Bolos**\n\
        • Bolo de Chocolate - R$ 45,00\n\
        • Bolo de Cenoura - R$ 40,00\n\
        • Bolo de Fubá - R$ 35,00\n\n\
        ?? **Salgados**\n\
        • Coxinha - R$ 6,50\n\
        • Pastel - R$ 7,00\n\
        • Esfiha - R$ 5,50\n\n\
        Digite o nome do produto para mais informações!".to_string()
    }
    
    fn product_info_response(&self) -> String {
        "Me diga qual produto você quer saber mais informações!".to_string()
    }
    
    fn product_availability_response(&self) -> String {
        "Para verificar disponibilidade, me diga qual produto você procura!".to_string()
    }
    
    fn product_price_response(&self) -> String {
        "Qual produto você gostaria de saber o preço?".to_string()
    }
    
    fn recommendations_response(&self) -> String {
        "?? **Recomendações do Dia**\n\n\
        Nossos produtos mais vendidos:\n\
        • Pão Francês quentinho - acabou de sair do forno! ??\n\
        • Bolo de Chocolate - favorito dos clientes\n\
        • Coxinha especial - crocante e saborosa\n\n\
        Quer adicionar algo ao pedido?".to_string()
    }
    
    fn business_hours_response(&self) -> String {
        "?? **Horário de Funcionamento**\n\n\
        Segunda a Sexta: 06:00 - 20:00\n\
        Sábado: 06:00 - 18:00\n\
        Domingo: 07:00 - 13:00\n\n\
        Estamos abertos agora! ??".to_string()
    }
    
    fn location_response(&self) -> String {
        "?? **Nossa Localização**\n\n\
        DelPopolo Panificadora\n\
        Rua das Padarias, 123\n\
        Centro - São Paulo/SP\n\
        CEP: 01234-567\n\n\
        [Ver no Google Maps]\n\
        Telefone: (11) 1234-5678".to_string()
    }
    
    fn delivery_area_response(&self) -> String {
        "?? **Área de Entrega**\n\n\
        Fazemos delivery nos seguintes bairros:\n\
        • Centro\n\
        • Jardins\n\
        • Pinheiros\n\
        • Vila Madalena\n\n\
        Taxa de entrega: R$ 5,00\n\
        Pedido mínimo: R$ 25,00\n\
        Tempo estimado: 30-45 minutos\n\n\
        Digite seu CEP para confirmar!".to_string()
    }
    
    fn current_promotions_response(&self) -> String {
        "?? **Promoções Ativas**\n\n\
        • Pão Francês: Leve 10, pague 8! ??\n\
        • Combo Café da Manhã: R$ 25,00\n\
          (6 pães + café + requeijão)\n\
        • Bolo do Dia: 20% OFF\n\n\
        Válido até hoje!\n\
        Quer aproveitar alguma promoção?".to_string()
    }
    
    fn special_offers_response(&self) -> String {
        self.current_promotions_response()
    }
    
    fn human_agent_response(&self) -> String {
        "Vou transferir você para um atendente humano.\n\
        Por favor, aguarde um momento... ?".to_string()
    }
    
    fn complaint_response(&self) -> String {
        "Sentimos muito pelo problema! ??\n\n\
        Sua satisfação é muito importante para nós.\n\
        Vou registrar sua reclamação e um gerente entrará em contato em breve.\n\n\
        Por favor, descreva o problema:".to_string()
    }
    
    fn compliment_response(&self) -> String {
        "Muito obrigado pelo elogio! ??\n\n\
        Ficamos muito felizes em saber que você gostou!\n\
        Nosso time trabalha com muito carinho para oferecer o melhor.\n\n\
        Ganhe 50 pontos de fidelidade! ??".to_string()
    }
    
    fn unknown_response(&self) -> String {
        "Desculpe, não entendi muito bem. ??\n\n\
        Posso ajudar com:\n\
        • Ver cardápio\n\
        • Fazer pedido\n\
        • Acompanhar pedido\n\
        • Promoções\n\
        • Horários e localização\n\n\
        Como posso ajudar?".to_string()
    }
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
