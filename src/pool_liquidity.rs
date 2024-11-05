// {
//     "statusCode": 200,
//     "data": {
//       "reserves": {
//         "mainToken": 39500145895827.49,
//         "sideToken": 0.032973238982557475
//       },
//       "liquidity": null
//     }
//   }

use serde::{Serialize, Deserialize};
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PoolLiquidity {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub data: PoolLiquidityData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PoolLiquidityData {
    pub reserves: PoolLiquidityReserves,
    pub liquidity: Option<f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PoolLiquidityReserves {
    #[serde(rename = "mainToken")]
    pub main_token: f64,
    #[serde(rename = "sideToken")]
    pub side_token: f64,
}

