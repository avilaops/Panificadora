/// Templates de emails HTML para notificações
pub struct EmailTemplates;

impl EmailTemplates {
    pub fn low_stock_alert(
        product_name: &str,
        current_stock: f64,
        min_stock: f64,
        supplier_name: Option<&str>,
        supplier_price: Option<f64>,
    ) -> String {
        let supplier_section = if let (Some(name), Some(price)) = (supplier_name, supplier_price) {
            format!(
                r#"
                <div style="margin-top: 20px; padding: 15px; background-color: #e3f2fd; border-left: 4px solid #2196f3;">
                    <h3 style="margin: 0 0 10px 0; color: #1976d2;">?? Fornecedor Recomendado</h3>
                    <p style="margin: 5px 0;"><strong>{}</strong></p>
                    <p style="margin: 5px 0; font-size: 18px; color: #1976d2;"><strong>R$ {:.2}</strong></p>
                </div>
                "#,
                name, price
            )
        } else {
            String::new()
        };
        
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Alerta de Estoque</title>
            </head>
            <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                    <div style="background-color: #ff5252; color: white; padding: 20px; border-radius: 5px;">
                        <h1 style="margin: 0;">?? Alerta de Estoque Baixo</h1>
                    </div>
                    
                    <div style="padding: 20px; background-color: #f5f5f5; margin-top: 20px; border-radius: 5px;">
                        <h2 style="color: #d32f2f; margin-top: 0;">Produto: {}</h2>
                        <table style="width: 100%; border-collapse: collapse;">
                            <tr>
                                <td style="padding: 10px 0;"><strong>Estoque Atual:</strong></td>
                                <td style="padding: 10px 0; text-align: right; color: #d32f2f; font-size: 18px;"><strong>{:.2}</strong></td>
                            </tr>
                            <tr>
                                <td style="padding: 10px 0;"><strong>Estoque Mínimo:</strong></td>
                                <td style="padding: 10px 0; text-align: right;">{:.2}</td>
                            </tr>
                            <tr>
                                <td colspan="2" style="padding: 10px 0;">
                                    <div style="background-color: #ff5252; color: white; padding: 5px 10px; border-radius: 3px; display: inline-block;">
                                        ABAIXO DO MÍNIMO
                                    </div>
                                </td>
                            </tr>
                        </table>
                    </div>
                    
                    {}
                    
                    <div style="margin-top: 30px; padding: 20px; background-color: #fff3e0; border-left: 4px solid #ff9800; border-radius: 5px;">
                        <p style="margin: 0; color: #e65100;"><strong>? Ação Recomendada:</strong></p>
                        <p style="margin: 10px 0 0 0;">Realizar pedido de reposição imediatamente para evitar falta de estoque.</p>
                    </div>
                    
                    <div style="margin-top: 30px; text-align: center; color: #666; font-size: 12px;">
                        <p>DelPopolo Panificadora - Sistema Automático de Gestão</p>
                        <p>panificadora.avila.inc</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            product_name, current_stock, min_stock, supplier_section
        )
    }
    
    pub fn order_confirmation(
        customer_name: &str,
        order_number: &str,
        items: &[(String, f64, f64)], // (nome, quantidade, preço)
        total: f64,
    ) -> String {
        let items_html: String = items
            .iter()
            .map(|(name, qty, price)| {
                format!(
                    r#"
                    <tr>
                        <td style="padding: 10px; border-bottom: 1px solid #ddd;">{}</td>
                        <td style="padding: 10px; border-bottom: 1px solid #ddd; text-align: center;">{:.0}</td>
                        <td style="padding: 10px; border-bottom: 1px solid #ddd; text-align: right;">R$ {:.2}</td>
                    </tr>
                    "#,
                    name, qty, price
                )
            })
            .collect();
        
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Confirmação de Pedido</title>
            </head>
            <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                    <div style="background-color: #4caf50; color: white; padding: 20px; border-radius: 5px;">
                        <h1 style="margin: 0;">? Pedido Confirmado!</h1>
                    </div>
                    
                    <div style="padding: 20px; margin-top: 20px;">
                        <p>Olá <strong>{}</strong>!</p>
                        <p>Seu pedido foi confirmado com sucesso e já está sendo preparado com muito carinho! ??</p>
                        
                        <div style="background-color: #f5f5f5; padding: 15px; border-radius: 5px; margin: 20px 0;">
                            <p style="margin: 0;"><strong>Número do Pedido:</strong></p>
                            <p style="font-size: 24px; color: #4caf50; margin: 10px 0; font-weight: bold;">{}</p>
                        </div>
                        
                        <h3>Itens do Pedido:</h3>
                        <table style="width: 100%; border-collapse: collapse;">
                            <thead>
                                <tr style="background-color: #f5f5f5;">
                                    <th style="padding: 10px; text-align: left;">Produto</th>
                                    <th style="padding: 10px; text-align: center;">Qtd</th>
                                    <th style="padding: 10px; text-align: right;">Valor</th>
                                </tr>
                            </thead>
                            <tbody>
                                {}
                            </tbody>
                            <tfoot>
                                <tr style="background-color: #4caf50; color: white;">
                                    <td colspan="2" style="padding: 15px; font-weight: bold;">TOTAL</td>
                                    <td style="padding: 15px; text-align: right; font-size: 18px; font-weight: bold;">R$ {:.2}</td>
                                </tr>
                            </tfoot>
                        </table>
                        
                        <div style="margin-top: 30px; padding: 15px; background-color: #e8f5e9; border-radius: 5px;">
                            <p style="margin: 0;">?? Você receberá uma notificação quando seu pedido estiver pronto para retirada!</p>
                        </div>
                    </div>
                    
                    <div style="margin-top: 30px; text-align: center; color: #666; font-size: 12px;">
                        <p>Obrigado por escolher a DelPopolo! ??</p>
                        <p>panificadora.avila.inc</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            customer_name, order_number, items_html, total
        )
    }
    
    pub fn thank_you(
        customer_name: &str,
        order_number: &str,
        loyalty_points: i32,
    ) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Obrigado!</title>
            </head>
            <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                    <div style="background-color: #9c27b0; color: white; padding: 30px; border-radius: 5px; text-align: center;">
                        <h1 style="margin: 0; font-size: 32px;">?? Muito Obrigado!</h1>
                    </div>
                    
                    <div style="padding: 30px 20px;">
                        <p style="font-size: 18px;">Olá <strong>{}</strong>!</p>
                        <p>Esperamos que você tenha aproveitado nossos produtos fresquinhos! ??</p>
                        <p>Foi um prazer atendê-lo no pedido <strong>{}</strong>.</p>
                        
                        <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 10px; text-align: center; margin: 30px 0;">
                            <p style="margin: 0; font-size: 14px; opacity: 0.9;">Você ganhou</p>
                            <p style="margin: 10px 0; font-size: 48px; font-weight: bold;">{}</p>
                            <p style="margin: 0; font-size: 14px; opacity: 0.9;">pontos de fidelidade! ??</p>
                        </div>
                        
                        <p style="text-align: center; color: #666;">Continue comprando e acumule benefícios especiais!</p>
                        
                        <div style="margin-top: 40px; text-align: center;">
                            <p style="color: #9c27b0; font-size: 18px; font-weight: bold;">Aguardamos sua próxima visita!</p>
                            <p style="color: #666; margin-top: 20px;">Com carinho,<br>Equipe DelPopolo ??</p>
                        </div>
                    </div>
                    
                    <div style="margin-top: 30px; text-align: center; color: #999; font-size: 11px;">
                        <p>DelPopolo Panificadora</p>
                        <p>panificadora.avila.inc</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            customer_name, order_number, loyalty_points
        )
    }
}
