use anyhow::Result;
use uuid::Uuid;
use tracing::{info, warn};
use delpopolo_domain::{Product, ProductCategory, UnitOfMeasure, Money, Supplier};
use super::models::{NFe, ItemNFe};

pub struct NFeImporter;

#[derive(Debug)]
pub struct ImportResult {
    pub nfe_key: String,
    pub supplier_created: bool,
    pub supplier_id: Uuid,
    pub products_created: Vec<Uuid>,
    pub products_updated: Vec<Uuid>,
    pub products_skipped: Vec<String>,
    pub total_items: usize,
}

impl NFeImporter {
    /// Importa produtos da NFe para o sistema
    /// Retorna resultado da importação com detalhes
    pub fn import_products_from_nfe(nfe: &NFe) -> Result<ImportResult> {
        info!("Importing products from NFe {}", nfe.chave);
        
        let mut result = ImportResult {
            nfe_key: nfe.chave.clone(),
            supplier_created: false,
            supplier_id: Uuid::new_v4(), // TODO: buscar ou criar fornecedor real
            products_created: Vec::new(),
            products_updated: Vec::new(),
            products_skipped: Vec::new(),
            total_items: nfe.itens.len(),
        };
        
        for item in &nfe.itens {
            match Self::create_product_from_item(item, nfe.chave.as_str()) {
                Ok(_product) => {
                    // TODO: Verificar se produto já existe por EAN/código
                    // Se existir, atualizar; se não, criar
                    result.products_created.push(Uuid::new_v4());
                }
                Err(e) => {
                    warn!("Failed to import item {}: {}", item.numero_item, e);
                    result.products_skipped.push(item.descricao.clone());
                }
            }
        }
        
        info!("Import completed: {} created, {} skipped", 
            result.products_created.len(), 
            result.products_skipped.len()
        );
        
        Ok(result)
    }
    
    fn create_product_from_item(item: &ItemNFe, nfe_key: &str) -> Result<Product> {
        let category = Self::map_ncm_to_category(&item.ncm);
        let unit = Self::map_unit(&item.unidade_comercial);
        
        let mut product = Product::new(
            item.descricao.clone(),
            category,
            unit,
            Money::brl(item.valor_unitario_comercial * 1.3), // Preço de venda com margem
            Money::brl(item.valor_unitario_comercial), // Custo
        );
        
        product.barcode = item.ean.clone();
        product.nfe_ncm = Some(item.ncm.clone());
        product.nfe_cest = item.cest.clone();
        product.nfe_cfop = Some(item.cfop.clone());
        product.stock_quantity = item.quantidade_comercial;
        
        Ok(product)
    }
    
    fn map_ncm_to_category(ncm: &str) -> ProductCategory {
        // Mapeamento básico de NCM para categorias
        // NCM 1905 = Produtos de padaria
        if ncm.starts_with("1905") {
            ProductCategory::Bread
        } else if ncm.starts_with("1806") {
            // Chocolate e cacau
            ProductCategory::Cake
        } else if ncm.starts_with("1904") {
            // Cereais
            ProductCategory::Snack
        } else if ncm.starts_with("2202") || ncm.starts_with("2203") {
            // Bebidas
            ProductCategory::Beverage
        } else {
            ProductCategory::RawMaterial
        }
    }
    
    fn map_unit(unit: &str) -> UnitOfMeasure {
        match unit.to_uppercase().as_str() {
            "UN" | "UNID" | "UNI" => UnitOfMeasure::Unit,
            "KG" | "KILO" => UnitOfMeasure::Kilogram,
            "G" | "GR" | "GRAMA" => UnitOfMeasure::Gram,
            "L" | "LT" | "LITRO" => UnitOfMeasure::Liter,
            "ML" | "MILILITRO" => UnitOfMeasure::Milliliter,
            "DZ" | "DUZIA" => UnitOfMeasure::Dozen,
            "PC" | "PCT" | "PACOTE" => UnitOfMeasure::Package,
            _ => UnitOfMeasure::Unit,
        }
    }
    
    pub fn create_supplier_from_nfe(nfe: &NFe) -> Supplier {
        let mut supplier = Supplier::new(nfe.emitente.razao_social.clone());
        
        supplier.trade_name = nfe.emitente.nome_fantasia.clone();
        supplier.cnpj = delpopolo_domain::Cnpj::new(nfe.emitente.cnpj.clone()).ok();
        supplier.email = nfe.emitente.email.as_ref()
            .and_then(|e| delpopolo_domain::Email::new(e.clone()).ok());
        supplier.phone = nfe.emitente.telefone.as_ref()
            .and_then(|p| delpopolo_domain::Phone::new(p.clone()).ok());
        
        let address = delpopolo_domain::Address::new(
            nfe.emitente.endereco.logradouro.clone(),
            nfe.emitente.endereco.numero.clone(),
            nfe.emitente.endereco.bairro.clone(),
            nfe.emitente.endereco.municipio.clone(),
            nfe.emitente.endereco.uf.clone(),
            nfe.emitente.endereco.cep.clone(),
        );
        
        if let Some(comp) = &nfe.emitente.endereco.complemento {
            supplier.address = Some(address.with_complement(comp.clone()));
        } else {
            supplier.address = Some(address);
        }
        
        supplier
    }
}
