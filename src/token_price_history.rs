use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TokenPriceHistory { 
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    pub data: TokenPriceHistoryData,
}

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct TokenPriceHistoryData {
    pub price: f64,
    #[serde(rename = "priceChain")]
    pub price_chain: Option<f64>,
    #[serde(rename = "price5m")]
    pub price_5m: Option<f64>,
    #[serde(rename = "priceChain5m")]
    pub price_chain_5m: Option<f64>,
    #[serde(rename = "variation5m")]
    pub variation_5m: Option<f64>,
    #[serde(rename = "variationChain5m")]
    pub variation_chain_5m: Option<f64>,
    #[serde(rename = "price1h")]
    pub price_1h: Option<f64>,
    #[serde(rename = "priceChain1h")]
    pub price_chain_1h: Option<f64>,
    #[serde(rename = "variation1h")]
    pub variation_1h: Option<f64>,
    #[serde(rename = "variationChain1h")]
    pub variation_chain_1h: Option<f64>,
    #[serde(rename = "price6h")]
    pub price_6h: Option<f64>,
    #[serde(rename = "priceChain6h")]
    pub price_chain_6h: Option<f64>,
    #[serde(rename = "variation6h")]
    pub variation_6h: Option<f64>,
    #[serde(rename = "variationChain6h")]
    pub variation_chain_6h: Option<f64>,
    #[serde(rename = "price24h")]
    pub price_24h: Option<f64>,
    #[serde(rename = "priceChain24h")]
    pub price_chain_24h: Option<f64>,
    #[serde(rename = "variation24h")]
    pub variation_24h: Option<f64>,
    #[serde(rename = "variationChain24h")]
    pub variation_chain_24h: Option<f64>,   
}



