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


// {
//     "list": [
//         {
//             "address": "0x31c6c8c2b1eabefb2fc27f5a28986e2fd0572f76",
//             "balance": "953512509617109549450551909",
//             "username": null,
//             "profile": null
//         },
//         {
//             "address": "0xff0f2decd8b5ef4a467510c353d1b56bfbfbf3c5",
//             "balance": "41697634508967779844282957",
//             "username": null,
//             "profile": null
//         },
//         {
//             "address": "0xa5000e209982fe9a78bdb25bbf4ad125cc857e07",
//             "balance": "2061945604175160967536561",
//             "username": "@VWXWVXWV",
//             "profile": null
//         },
//         {
//             "address": "0x724bf6bb016a9eb82fa89ab2cf9c94bcd520a8af",
//             "balance": "1904252452492135050870208",
//             "username": "@PetrOsetr",
//             "profile": null
//         },
//         {
//             "address": "0x4444682b8892bb1d42dea9329c3b620667f603e8",
//             "balance": "617020874586906712699423",
//             "username": null,
//             "profile": null
//         },
//         {
//             "address": "0x777336ae2cef9ddc261a61a97cbfb4e0aa7d1329",
//             "balance": "206636554604711932383675",
//             "username": null,
//             "profile": null
//         },
//         {
//             "address": "0xcd68b075ac5ce995a5dc03d2be5d0ae58f6c9a6f",
//             "balance": "320466959556851453",
//             "username": null,
//             "profile": null
//         },
//         {
//             "address": "0xbd8e12d1a2119e86e3714dd121e46c45ccbefc54",
//             "balance": "67596796484823781",
//             "username": "@apeapeape",
//             "profile": null
//         },
//         {
//             "address": "0x1491622ffb8becb8ecc981e228bf861800d228db",
//             "balance": "17",
//             "username": "@MachiBigBrother",
//             "profile": "https://hf23bp85rhzueieu.public.blob.vercel-storage.com/users/0x1491622ffb8becb8ecc981e228bf861800d228db-uJIWk1YcSfPnZO31eb1LtkVAvsz3ZT"
//         },
//         {
//             "address": "0x1b3c0c4ef262f0325ea49cde1a21391e509dee7f",
//             "balance": "16",
//             "username": "@brush_bots",
//             "profile": "https://hf23bp85rhzueieu.public.blob.vercel-storage.com/users/0x1b3c0c4ef262f0325ea49cde1a21391e509dee7f-hOJd7I5eTShsfcVrQiw7Pfol8SXSri"
//         }
//     ],
//     "totalHolders": "10"
// }