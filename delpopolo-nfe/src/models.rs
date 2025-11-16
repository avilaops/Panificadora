use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFe {
    pub chave: String, // Chave de acesso de 44 dígitos
    pub numero: String,
    pub serie: String,
    pub data_emissao: DateTime<Utc>,
    pub emitente: Emitente,
    pub destinatario: Destinatario,
    pub itens: Vec<ItemNFe>,
    pub totais: Totais,
    pub transporte: Option<Transporte>,
    pub informacoes_adicionais: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emitente {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub endereco: Endereco,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destinatario {
    pub cnpj_cpf: String,
    pub razao_social: String,
    pub endereco: Endereco,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endereco {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub municipio: String,
    pub uf: String,
    pub cep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemNFe {
    pub numero_item: i32,
    pub codigo_produto: String,
    pub descricao: String,
    pub ncm: String,
    pub cest: Option<String>,
    pub cfop: String,
    pub unidade_comercial: String,
    pub quantidade_comercial: f64,
    pub valor_unitario_comercial: f64,
    pub valor_total_bruto: f64,
    pub ean: Option<String>,
    pub ean_tributavel: Option<String>,
    pub origem: Option<String>,
    pub icms: Option<ICMS>,
    pub ipi: Option<IPI>,
    pub pis: Option<PIS>,
    pub cofins: Option<COFINS>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICMS {
    pub situacao_tributaria: String,
    pub base_calculo: Option<f64>,
    pub aliquota: Option<f64>,
    pub valor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPI {
    pub situacao_tributaria: String,
    pub base_calculo: Option<f64>,
    pub aliquota: Option<f64>,
    pub valor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIS {
    pub situacao_tributaria: String,
    pub base_calculo: Option<f64>,
    pub aliquota: Option<f64>,
    pub valor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct COFINS {
    pub situacao_tributaria: String,
    pub base_calculo: Option<f64>,
    pub aliquota: Option<f64>,
    pub valor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Totais {
    pub base_calculo_icms: f64,
    pub valor_icms: f64,
    pub valor_icms_desonerado: f64,
    pub base_calculo_icms_st: f64,
    pub valor_icms_st: f64,
    pub valor_total_produtos: f64,
    pub valor_frete: f64,
    pub valor_seguro: f64,
    pub valor_desconto: f64,
    pub valor_total_ii: f64,
    pub valor_ipi: f64,
    pub valor_pis: f64,
    pub valor_cofins: f64,
    pub valor_outras_despesas: f64,
    pub valor_total_nota: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transporte {
    pub modalidade: String,
    pub transportadora: Option<Transportadora>,
    pub veiculo: Option<Veiculo>,
    pub volumes: Vec<Volume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transportadora {
    pub cnpj_cpf: Option<String>,
    pub razao_social: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub endereco: Option<String>,
    pub municipio: Option<String>,
    pub uf: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Veiculo {
    pub placa: String,
    pub uf: String,
    pub rntc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub quantidade: i32,
    pub especie: Option<String>,
    pub marca: Option<String>,
    pub numeracao: Option<String>,
    pub peso_liquido: Option<f64>,
    pub peso_bruto: Option<f64>,
}

impl NFe {
    pub fn total_itens(&self) -> usize {
        self.itens.len()
    }
    
    pub fn valor_total(&self) -> f64 {
        self.totais.valor_total_nota
    }
    
    pub fn is_entrada(&self) -> bool {
        // Determina se é nota de entrada (compra) baseado no CFOP
        self.itens.iter().any(|item| {
            item.cfop.starts_with('1') || item.cfop.starts_with('2') || item.cfop.starts_with('3')
        })
    }
}
