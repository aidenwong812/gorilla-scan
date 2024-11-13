use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HolderInfo {
    pub address: String,
    pub balance: String,
    pub username: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenTopHolders {
    pub list: Vec<HolderInfo>,
    #[serde(rename = "totalHolders")]
    pub total_holders: String,
}

// When deserializing, you'll need a custom implementation:
impl From<Vec<(String, String, Option<String>, Option<String>)>> for TokenTopHolders {
    fn from(list: Vec<(String, String, Option<String>, Option<String>)>) -> Self {
        let holders = list
            .into_iter()
            .map(|(addr, amount, username, profile)| HolderInfo {
                address: addr,
                balance: amount,
                username: username,
                profile: profile,
            })
            .collect();
        TokenTopHolders { list: holders, total_holders: "".to_string() }
    }
}


