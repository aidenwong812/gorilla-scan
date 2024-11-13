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

// {
//     "id": "0x6874c70e43657fe9c6aee57d200db949e093b127",
//     "launchAt": null,
//     "name": "MarsCoin",
//     "symbol": "MARSCOIN",
//     "totalSupply": "1000000000000000000000000000",
//     "totalBurned": "0",
//     "creator": "0xff0f2decd8b5ef4a467510c353d1b56bfbfbf3c5",
//     "blockTimestamp": "1731478337",
//     "lootCounter": "1",
//     "bondingCurve": {
//         "id": "0x31c6c8c2b1eabefb2fc27f5a28986e2fd0572f76",
//         "router": "0x18e621b64d7808c3c47bccbbd7485d23f257d26f",
//         "virtualAPEReserve": "5248913139899053657933",
//         "virtualTokenReserve": "1026512509617109549450551909",
//         "realAPEReserve": "227408014083583014877",
//         "realTokenReserve": "953512509617109549450551909",
//         "initialVirtualAPE": "5021505125815470643056",
//         "finalVirtualAPE": "19250000000000000000000",
//         "tradeFeePercent": "10",
//         "totalTradeFees": "23880020757528678760",
//         "apxSuccessFee": "0",
//         "creatorSuccessFee": "0",
//         "kingOfTheHillTimestamp": null
//     },
//     "liquidity": liquidity\":{\"pair\":\"0xb064ea265e262ea2d21645ff5aa127c3088955ee\",\"router\":\"0x18e621b64d7808c3c47bccbbd7485d23f257d26f\",\"nativeReserve\":\"126128482207381064954175\",\"tokenReserve\":\"80242581834216543780633052\",\"initialNativeReserve\":\"18478564771668219944083\",\"initialTokenReserve\":\"206900000000000000000000000\",\"isToken0\":false,\"id\":\"0xb064ea265e262ea2d21645ff5aa127c3088955ee\"}
//     "isProfane": false,
//     "details": {
//         "description": "\"It is necessary. Maybe call it MarsCoin?\" - CZ\n\"There will definitely be a MarsCoin!\" - Elon Musk",
//         "telegram": "",
//         "twitter": "",
//         "website": "",
//         "discord": "",
//         "user": {
//             "address": "0xff0f2decd8b5ef4a467510c353d1b56bfbfbf3c5",
//             "username": null,
//             "profile": null
//         },
//         "isProfane": false
//     },
//     "price": "0.000005113345517685"
// }