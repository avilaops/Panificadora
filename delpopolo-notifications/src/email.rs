use lettre::{
    Message, SmtpTransport, Transport,
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use anyhow::Result;
use tracing::{info, error};

pub struct EmailService {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from_address: String,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_address: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_address,
        }
    }
    
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body_text: &str,
        body_html: Option<&str>,
    ) -> Result<()> {
        info!("Sending email to {} - Subject: {}", to, subject);
        
        let mut email_builder = Message::builder()
            .from(self.from_address.parse()?)
            .to(to.parse()?)
            .subject(subject);
        
        let email = if let Some(html) = body_html {
            email_builder.multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(body_text.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html.to_string()),
                    ),
            )?
        } else {
            email_builder.body(body_text.to_string())?
        };
        
        let creds = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );
        
        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .port(self.smtp_port)
            .credentials(creds)
            .build();
        
        match mailer.send(&email) {
            Ok(_) => {
                info!("Email sent successfully to {}", to);
                Ok(())
            }
            Err(e) => {
                error!("Failed to send email to {}: {}", to, e);
                Err(anyhow::anyhow!("Failed to send email: {}", e))
            }
        }
    }
    
    /// Envia email de alerta de estoque baixo
    pub async fn send_low_stock_alert(
        &self,
        to: &str,
        product_name: &str,
        current_stock: f64,
        min_stock: f64,
        supplier_name: Option<&str>,
        supplier_price: Option<f64>,
    ) -> Result<()> {
        let subject = format!("?? Alerta: Estoque baixo de {}", product_name);
        
        let supplier_info = if let (Some(name), Some(price)) = (supplier_name, supplier_price) {
            format!("\n\nFornecedor recomendado: {}\nPreço: R$ {:.2}", name, price)
        } else {
            String::new()
        };
        
        let body = format!(
            "ALERTA DE ESTOQUE BAIXO\n\n\
            Produto: {}\n\
            Estoque atual: {:.2}\n\
            Estoque mínimo: {:.2}\n\
            Status: ABAIXO DO MÍNIMO{}\n\n\
            Ação recomendada: Realizar pedido de reposição imediatamente.\n\n\
            --\n\
            DelPopolo Panificadora\n\
            Sistema Automático de Gestão",
            product_name, current_stock, min_stock, supplier_info
        );
        
        self.send_email(to, &subject, &body, None).await
    }
    
    /// Envia email de confirmação de pedido
    pub async fn send_order_confirmation(
        &self,
        to: &str,
        customer_name: &str,
        order_number: &str,
        total: f64,
    ) -> Result<()> {
        let subject = format!("? Pedido {} confirmado", order_number);
        
        let body = format!(
            "Olá {}!\n\n\
            Seu pedido foi confirmado com sucesso!\n\n\
            Número do pedido: {}\n\
            Valor total: R$ {:.2}\n\n\
            Você receberá uma notificação quando seu pedido estiver pronto.\n\n\
            Obrigado por escolher a DelPopolo! ??\n\n\
            --\n\
            DelPopolo Panificadora\n\
            panificadora.avila.inc",
            customer_name, order_number, total
        );
        
        self.send_email(to, &subject, &body, None).await
    }
    
    /// Envia email de agradecimento
    pub async fn send_thank_you_email(
        &self,
        to: &str,
        customer_name: &str,
        order_number: &str,
        loyalty_points: i32,
    ) -> Result<()> {
        let subject = "?? Obrigado pela sua compra!";
        
        let body = format!(
            "Olá {}!\n\n\
            Muito obrigado pela sua compra! #{}\n\n\
            Esperamos que você tenha aproveitado nossos produtos fresquinhos! ??\n\n\
            Você ganhou {} pontos de fidelidade nesta compra!\n\
            Continue comprando e acumule benefícios especiais.\n\n\
            Aguardamos sua próxima visita!\n\n\
            Com carinho,\n\
            Equipe DelPopolo ??\n\n\
            --\n\
            DelPopolo Panificadora\n\
            panificadora.avila.inc",
            customer_name, order_number, loyalty_points
        );
        
        self.send_email(to, &subject, &body, None).await
    }
}
