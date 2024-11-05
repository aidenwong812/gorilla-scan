use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HolderInfo {
    pub holder_address: String,
    pub usd_amount: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenTopHolders {
    pub holders: Vec<HolderInfo>,
}

// When deserializing, you'll need a custom implementation:
impl From<Vec<(String, f64)>> for TokenTopHolders {
    fn from(data: Vec<(String, f64)>) -> Self {
        let holders = data
            .into_iter()
            .map(|(addr, amount)| HolderInfo {
                holder_address: addr,
                usd_amount: amount,
            })
            .collect();
        TokenTopHolders { holders }
    }
}
