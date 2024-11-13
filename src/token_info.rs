use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "id")]
    pub address: String,
    #[serde(rename = "launchAt")]
    pub launch_at: Option<String>,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
    #[serde(rename = "totalBurned")]
    pub total_burned: String,
    pub creator: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "lootCounter")]
    pub loot_counter: String,
    pub bonding_curve: Option<BondingCurve>,
    pub liquidity: Option<Liquidity>,
    #[serde(rename = "isProfane")]
    pub is_profane: bool,
    pub details: Option<Details>,
    pub price: String,
}

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct BondingCurve {
    pub id: String,
    pub router: String,
    #[serde(rename = "virtualAPEReserve")]
    pub virtual_ape_reserve: String,
    #[serde(rename = "virtualTokenReserve")]
    pub virtual_token_reserve: String,
    #[serde(rename = "realAPEReserve")]
    pub real_ape_reserve: String,
    #[serde(rename = "realTokenReserve")]
    pub real_token_reserve: String,
    #[serde(rename = "initialVirtualAPE")]
    pub initial_virtual_ape: String,
    #[serde(rename = "finalVirtualAPE")]
    pub final_virtual_ape: String,
    #[serde(rename = "tradeFeePercent")]
    pub trade_fee_percent: String,
    #[serde(rename = "totalTradeFees")]
    pub total_trade_fees: String,
    #[serde(rename = "apxSuccessFee")]
    pub apx_success_fee: String,
    #[serde(rename = "creatorSuccessFee")]
    pub creator_success_fee: String,
    #[serde(rename = "kingOfTheHillTimestamp")]
    pub king_of_the_hill_timestamp: Option<String>,
}

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct Details {
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub user: Option<User>,
    #[serde(rename = "isProfane")]
    pub is_profane: bool,
}

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct User {
    pub address: String,
    pub username: Option<String>,
    pub profile: Option<String>,
}

#[derive(Default, Debug, Clone,  Serialize, Deserialize)]
pub struct Liquidity {
    pub pair: String,
    pub router: String,
    #[serde(rename = "nativeReserve")]
    pub native_reserve: String,
    #[serde(rename = "tokenReserve")]
    pub token_reserve: String,
    #[serde(rename = "initialNativeReserve")]
    pub initial_native_reserve: String,
    #[serde(rename = "initialTokenReserve")]
    pub initial_token_reserve: String,
    #[serde(rename = "isToken0")]
    pub is_token0: bool,
    pub id: String,
}

