use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TokenAudit {
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    pub data: TokenAuditData,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TokenAuditData {
    #[serde(rename = "isOpenSource")]
    pub is_open_source: String,
    #[serde(rename = "isHoneypot")]
    pub is_honeypot: String,
    #[serde(rename = "isMintable")]
    pub is_mintable: String,
    #[serde(rename = "isProxy")]
    pub is_proxy: String,
    #[serde(rename = "slippageModifiable")]
    pub slippage_modifiable: String,
    #[serde(rename = "isBlacklisted")]
    pub is_blacklisted: String,
    #[serde(rename = "sellTax")]
    pub sell_tax: Tax,
    #[serde(rename = "buyTax")]
    pub buy_tax: Tax,
    #[serde(rename = "isContractRenounced")]
    pub is_contract_renounced: String,
    #[serde(rename = "isPotentiallyScam")]
    pub is_potentially_scam: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String, 
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]    
pub struct Tax {
    pub min: f64,
    pub max: f64,
    pub status: String,
}

// impl TokenAudit {
//     pub fn data(&self) -> &TokenAuditData {
//         &self.data
//     }
// }

