use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenPool {
    pub data: TokenPooldata,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenPooldata {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    pub results: Vec<Pool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pool {
    #[serde(rename = "creationBlock")]
    pub creation_block: i32,
    #[serde(rename = "creationTime")]
    pub creation_time: String,
    pub exchange: Exchange,
    #[serde(rename = "mainToken")]
    pub main_token: Token,
    #[serde(rename = "sideToken")]
    pub side_token: Token,
    pub fee: Option<i32>,
    pub address: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Exchange {
    pub name: String,
    pub factory: String,
}
