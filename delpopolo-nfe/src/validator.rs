use anyhow::Result;
use super::models::NFe;

pub struct NFeValidator;

impl NFeValidator {
    pub fn validate(nfe: &NFe) -> Result<()> {
        Self::validate_chave(&nfe.chave)?;
        Self::validate_emitente(&nfe)?;
        Self::validate_itens(&nfe)?;
        Self::validate_totais(&nfe)?;
        
        Ok(())
    }
    
    fn validate_chave(chave: &str) -> Result<()> {
        if chave.len() != 44 {
            anyhow::bail!("Chave de acesso deve ter 44 dígitos");
        }
        
        if !chave.chars().all(|c| c.is_numeric()) {
            anyhow::bail!("Chave de acesso deve conter apenas números");
        }
        
        Ok(())
    }
    
    fn validate_emitente(nfe: &NFe) -> Result<()> {
        if nfe.emitente.cnpj.is_empty() {
            anyhow::bail!("CNPJ do emitente é obrigatório");
        }
        
        if nfe.emitente.razao_social.is_empty() {
            anyhow::bail!("Razão social do emitente é obrigatória");
        }
        
        Ok(())
    }
    
    fn validate_itens(nfe: &NFe) -> Result<()> {
        if nfe.itens.is_empty() {
            anyhow::bail!("NFe deve ter pelo menos um item");
        }
        
        for item in &nfe.itens {
            if item.descricao.is_empty() {
                anyhow::bail!("Descrição do item {} está vazia", item.numero_item);
            }
            
            if item.quantidade_comercial <= 0.0 {
                anyhow::bail!("Quantidade do item {} deve ser maior que zero", item.numero_item);
            }
            
            if item.valor_unitario_comercial <= 0.0 {
                anyhow::bail!("Valor unitário do item {} deve ser maior que zero", item.numero_item);
            }
        }
        
        Ok(())
    }
    
    fn validate_totais(nfe: &NFe) -> Result<()> {
        let soma_itens: f64 = nfe.itens.iter()
            .map(|item| item.valor_total_bruto)
            .sum();
        
        let diferenca = (soma_itens - nfe.totais.valor_total_produtos).abs();
        
        if diferenca > 0.01 {
            anyhow::bail!(
                "Soma dos itens ({}) não confere com total de produtos ({})",
                soma_itens,
                nfe.totais.valor_total_produtos
            );
        }
        
        Ok(())
    }
    
    pub fn is_valid_for_import(nfe: &NFe) -> bool {
        // Verifica se é nota de entrada (CFOP iniciando com 1, 2 ou 3)
        nfe.is_entrada() && nfe.itens.iter().all(|item| {
            !item.descricao.is_empty() && 
            item.quantidade_comercial > 0.0 &&
            item.valor_unitario_comercial > 0.0
        })
    }
}
