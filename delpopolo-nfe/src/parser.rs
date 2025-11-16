use quick_xml::de::from_str;
use anyhow::Result;
use tracing::{info, error};
use super::models::NFe;

pub struct NFeParser;

impl NFeParser {
    pub fn parse_xml(xml_content: &str) -> Result<NFe> {
        info!("Parsing NFe XML");
        
        // Simplificação: na prática, o XML da NFe é complexo e aninhado
        // Aqui faremos um parsing básico que pode ser expandido
        
        match from_str::<NFe>(xml_content) {
            Ok(nfe) => {
                info!("NFe parsed successfully. Key: {}", nfe.chave);
                Ok(nfe)
            }
            Err(e) => {
                error!("Failed to parse NFe XML: {}", e);
                Err(anyhow::anyhow!("NFe parsing error: {}", e))
            }
        }
    }
    
    pub fn parse_from_file(file_path: &str) -> Result<NFe> {
        info!("Reading NFe from file: {}", file_path);
        
        let xml_content = std::fs::read_to_string(file_path)?;
        Self::parse_xml(&xml_content)
    }
    
    pub fn extract_chave_from_barcode(barcode: &str) -> Option<String> {
        // Código de barras da NFe tem 44 dígitos
        // Formato: chave de acesso
        if barcode.len() == 44 && barcode.chars().all(|c| c.is_numeric()) {
            Some(barcode.to_string())
        } else {
            None
        }
    }
    
    pub fn validate_chave(chave: &str) -> bool {
        if chave.len() != 44 {
            return false;
        }
        
        if !chave.chars().all(|c| c.is_numeric()) {
            return false;
        }
        
        // Validação do dígito verificador (posição 44)
        let digits: Vec<u32> = chave.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        let check_digit = Self::calculate_check_digit(&digits[0..43]);
        
        digits[43] == check_digit
    }
    
    fn calculate_check_digit(digits: &[u32]) -> u32 {
        let weights = [4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut sum = 0u32;
        
        for (i, &digit) in digits.iter().enumerate() {
            sum += digit * weights[i % weights.len()];
        }
        
        let remainder = sum % 11;
        
        if remainder == 0 || remainder == 1 {
            0
        } else {
            11 - remainder
        }
    }
    
    pub fn extract_supplier_info(nfe: &NFe) -> (String, Option<String>, Option<String>) {
        (
            nfe.emitente.razao_social.clone(),
            Some(nfe.emitente.cnpj.clone()),
            nfe.emitente.email.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_chave() {
        let valid_chave = "35250112345678000190550010000000011234567890";
        assert!(NFeParser::validate_chave(valid_chave));
        
        let invalid_chave = "12345678901234567890123456789012345678901234";
        assert!(!NFeParser::validate_chave(invalid_chave));
    }
}
