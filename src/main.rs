pub mod token_info;
pub mod token_price_history;
pub mod token_holders;
pub mod token_audit;
pub mod native_token;

use dotenv::dotenv;
use log::error;
use reqwest::Client;
use serde_json;
use std::env;
use teloxide::{
    prelude::*, requests::JsonRequest, types::{Me, MessageKind, ParseMode, TextQuote}, utils::command::BotCommands
};
use teloxide::types::LinkPreviewOptions;
use chrono::{DateTime, Utc};
use token_info::*;
use token_price_history::*;
use token_holders::*;
use token_audit::*;
use native_token::*;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help message")]
    Help,
    #[command(description = "Send the welcome message")]
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let bot_commands = Command::bot_commands();
    if bot.set_my_commands(bot_commands).await.is_err() {
        log::warn!("Could not set up the commands.");
    }

    Dispatcher::builder(
        bot,
        dptree::entry().branch(Update::filter_message().endpoint(message_handler)),
    )
    .build()
    .dispatch()
    .await;

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    dotenv().ok();

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {

            let chat_type = match msg.chat.kind {
                teloxide::types::ChatKind::Private { .. } => {
                    "a private chat".to_string()
                }
                teloxide::types::ChatKind::Public(ref public_chat) => {
                    match public_chat.kind {
                        teloxide::types::PublicChatKind::Group { .. } => "a group".to_string(),
                        teloxide::types::PublicChatKind::Supergroup { .. } => "a supergroup".to_string(),
                        teloxide::types::PublicChatKind::Channel { .. } => "a channel".to_string(),
                    }
                }
            };

            if chat_type == "a group" || chat_type == "a supergroup" {
                let username = msg.from()
                                    .and_then(|user| user.username.clone())
                                    .unwrap_or_else(|| {
                                        msg.from()
                                            .map(|user| user.first_name.clone())
                                            .unwrap_or_else(|| "Unknown User".to_string())
                                    });
                if let Ok(cmd) = Command::parse(text, me.username()) {
                    answer_command(bot, msg, cmd, username).await?;
                } else {
                    answer_message(bot, msg).await?;
                }
            } else {
                bot.send_message(msg.chat.id, "This message is not available in this chat type. Please try again in group chat.")
                    .await?;
        }
        
    }

    Ok(())
}

async fn answer_command(bot: Bot, msg: Message, cmd: Command, username: String) -> ResponseResult<()> {

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, format!("Welcome to Here @{username}! 🎉"))
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Invalid command")
                .await?;
        }
    }
    Ok(())
}

async fn answer_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let token_adr = msg.text().unwrap();
    if token_adr.starts_with("0x") && token_adr.len() == 42 && token_adr[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                
        let request_client = Client::new();
        let dextools_api_key = env::var("DEXTOOLS_API_KEY").expect("API_KEY not set");
        let dextools_api_plan = env::var("DEXTOOLS_API_PLAN").expect("API_PLAN not set");
        
        match get_token_info(request_client.clone(), &token_adr).await {
            Ok(token_info) => {
                let token_price_history = get_token_price_history(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await.unwrap_or_default();
                let token_holders = get_holders(request_client.clone(), &token_adr).await.unwrap_or_default();
                let token_audit = get_token_audit(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await.unwrap_or_default();
                //make message
                let text =
                make_token_overview_message(&token_info, &token_price_history, &token_holders, &token_audit)
                        .await?;
                bot.send_message(msg.chat.id, text)  // Changed "text" to text
                        .parse_mode(ParseMode::Html)
                        .link_preview_options(LinkPreviewOptions {
                            is_disabled: true,
                            prefer_small_media: false,
                            prefer_large_media: false,
                            show_above_text: false,
                            url: None,
                        })
                        .send()
                        .await?;
            }
            Err(e) => {
                error!("Error fetching token overview: {}", e);
                bot.send_message(msg.chat.id, "Invalid token address")
                .await?;
            }
        }
    }
    Ok(())
}


async fn get_token_info(client: Client, token_address: &str) -> Result<TokenInfo, serde_json::Error> {
    let url = format!(
        "https://ape.express/api/tokens/{}",
        token_address
    );

    let response = client
    .get(&url)
    .send()
    .await
    .unwrap();

    println!("Response: {:?}", response);
    
    let text = match response.text().await {
        Ok(text) => {text},
        Err(e) => {
            println!("Error fetching token overview: {}", e);
            // format!(e.to_string());
            "Error fetching token overview: {e}".to_string()
        }
    };
    match serde_json::from_str(&text) {
        Ok(obj) => {
            Ok(obj)
        },
        Err(e) =>  {
            println!("Error parsing JSON: {}", e);
            Err(e)
        },
    }
}

async fn get_token_price_history(client: Client, api_key: &str, api_plan: &str, token_address: &str) -> Result<TokenPriceHistory, serde_json::Error> {
    let url = format!(
        "https://public-api.dextools.io/{}/v2/token/{}/{}/price",
        api_plan, "apechain", token_address
    );

    let response = client
    .get(&url)
    .header("X-API-KEY", api_key)
    .send()
    .await
    .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) =>  Err(e),
    }
}

async fn get_token_audit(client: Client, api_key: &str, api_plan: &str, token_address: &str) -> Result<TokenAudit, serde_json::Error> {
    let url = format!(
        "https://public-api.dextools.io/{}/v2/token/{}/{}/audit",
        api_plan, "apechain", token_address
    );
    let response = client
    .get(&url)
    .header("X-API-KEY", api_key)
    .send()
    .await
    .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) =>  Err(e),
    }
}

async fn get_holders(
    client: Client,
    token_address: &str,
) -> Result<TokenTopHolders, serde_json::Error> {
    let url = format!(
        "https://ape.express/api/tokens/{}/holders",
        token_address
    );
    
    let response = client
        .get(&url)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    
    let holders:TokenTopHolders = serde_json::from_str(&text).unwrap_or_default();
    Ok(holders)
}

async fn make_token_overview_message(
    token_info: &TokenInfo,
    token_price_history: &TokenPriceHistory,
    token_top_holders: &TokenTopHolders,
    token_audit: &TokenAudit,
) -> Result<String, reqwest::Error> {

    let token_decimal = 18;
    
    // Get native token price
    let native_token_price = match get_native_token_price().await {
        Ok(token) => token.price.parse::<f64>().unwrap_or_default() / 10_f64.powi(8),
        Err(_) => 0.0,
    };

    // Extract token info with proper error handling
    let token_address = &token_info.address;
    // let token_launch_at = &token_info.launch_at;
    let token_name = &token_info.name;
    let token_symbol = &token_info.symbol;
    let token_total_supply = token_info.total_supply.parse::<f64>().unwrap_or_default() / 10_f64.powi(token_decimal as i32); 
    let token_block_timestamp = &token_info.block_timestamp;
    let token_price = num_floating_point(&(token_info.price.parse::<f64>().unwrap_or_default() * native_token_price), 5);
    let token_liquidity = token_info.liquidity.clone().unwrap_or_default();
    let liquidity = controll_big_float(token_liquidity.native_reserve.parse::<f64>().unwrap_or_default() / 10_f64.powi(token_decimal as i32) * native_token_price * 2.0);
    
    let market_cap = controll_big_float(token_total_supply * token_price);
    let age = if let Some(block_time) = token_block_timestamp {
        calculate_age(block_time)
    } else {
        "🔥".to_string()
    };


    //social info
    let mut social_text = String::new();
    if let Some(details) = &token_info.details {
        if let Some(discord) = &details.discord {
            if !discord.is_empty() {
                social_text += &format!(" <a href=\"{discord}\">💭 </a>");
            }
        }
        if let Some(telegram) = &details.telegram {
            if !telegram.is_empty() {
                social_text += &format!(" <a href=\"{telegram}\">🕊️ </a>");
            }
        }
        if let Some(twitter) = &details.twitter {
            if !twitter.is_empty() {
                social_text += &format!(" <a href=\"{twitter}\">𝕏 </a>");
            }
        }
        if let Some(website) = &details.website {
            if !website.is_empty() {
                social_text += &format!(" <a href=\"{website}\">🌐 </a>");
            }
        }
    }
   
    //top price history
    // let price = num_floating_point(&token_price_history.data.price, 3)  ;
    let price_1h = num_floating_point(&token_price_history.data.price_1h.unwrap_or_default(), 3);
    let price_6h = num_floating_point(&token_price_history.data.price_6h.unwrap_or_default(), 3);
    let price_24h = num_floating_point(&token_price_history.data.price_24h.unwrap_or_default(), 3);
    let variation_1h = num_floating_point(&token_price_history.data.variation_1h.unwrap_or_default(), 2);
    let variation_6h = num_floating_point(&token_price_history.data.variation_6h.unwrap_or_default(), 2);
    let variation_24h = num_floating_point(&token_price_history.data.variation_24h.unwrap_or_default(), 2);

    //top holders Info
     let holders_count = token_top_holders.total_holders.parse::<u32>().unwrap_or_default();
     let mut sum_usd_amount_top_10_holders = 0.0;
     let mut holders_text = String::from("\n");
     let mut top_num = 0;
     let mut index_on_a_line = 0;
     let mut num_whale = 0;
     let mut num_largefish = 0;
     let mut num_bigfish = 0;
     let mut num_smallfish = 0;
     let mut num_shrimp = 0;
    
     if holders_count >= 50  {
        holders_text += &format!("<u><b><i>50 Top Holders Map</i></b></u>\n        ");
     } else if holders_count > 0{
        holders_text += &format!("<u><b><i>{holders_count} Top Holders Map</i></b></u>\n        ");
     }
     for holder in &token_top_holders.list {
         let holder_address = &holder.address;
         let balance = holder.balance.parse::<f64>().unwrap_or_default();
         let usd_amount = balance / 10_f64.powi(token_decimal as i32) * token_price;
 
         top_num += 1;
         if top_num <= 10 {
             sum_usd_amount_top_10_holders += usd_amount;
         }
 
         let whale_symbol = if usd_amount > 100000.0 {
             num_whale += 1;
             "🐳"
         } else if usd_amount > 50000.0 {
             num_largefish += 1;
             "🦈"
         } else if usd_amount > 10000.0 {
             num_bigfish += 1;
             "🐬"
         } else if usd_amount > 1000.0 {
             num_smallfish += 1;
             "🐟"
         } else {
             num_shrimp += 1;
             "🦐"
         };
 
         let link = format!("<a href=\"https://apescan.io/address/{holder_address}?Amount={usd_amount}\">{whale_symbol}</a>");
         if index_on_a_line == 9 {
             holders_text = holders_text + &link + "\n        ";
             index_on_a_line = 0;
         } else {
             holders_text = holders_text + &link;
             index_on_a_line += 1;
         }
         if holders_count <= 50 {
            if top_num == holders_count {
                holders_text += &format!("\n        🐳 ( > $100K ) :  {num_whale}\n        🦈 ( $50K - $100K ) :  {num_largefish}\n        🐬 ( $10K - $50K ) :  {num_bigfish}\n        🐟 ( $1K - $10K ) :  {num_smallfish}\n        🦐 ( $0 - $1K ) :  {num_shrimp}\n");
            }
         } else {
            if top_num == 50 {
                holders_text += &format!("\n        🐳 ( > $100K ) :  {num_whale}\n        🦈 ( $50K - $100K ) :  {num_largefish}\n        🐬 ( $10K - $50K ) :  {num_bigfish}\n        🐟 ( $1K - $10K ) :  {num_smallfish}\n        🦐 ( $0 - $1K ) :  {num_shrimp}\n");
                break;
            }
         }
     }
    let sum_usd_amount_top_10_holders = controll_big_float(sum_usd_amount_top_10_holders);

    //token audit
    let mut audit_text = String::new();
    let token_audit_status = &token_audit.status_code;
    if *token_audit_status == 200 {
        let is_open_source = &token_audit.data.is_open_source;
        let is_honeypot = &token_audit.data.is_honeypot;
        let is_mintable = &token_audit.data.is_mintable;
        let is_proxy = &token_audit.data.is_proxy;
        let slippage_modifiable = &token_audit.data.slippage_modifiable;
        let is_blacklisted = &token_audit.data.is_blacklisted;
        let is_contract_renounced = &token_audit.data.is_contract_renounced;
        let is_potentially_scam = &token_audit.data.is_potentially_scam;

        audit_text += &format!("🔍 Audit\n");
        if is_open_source    == "yes" {
            audit_text += &format!("        🔓 Open source: ✅\n");
        } else if is_open_source == "no" {
            audit_text += &format!("        🔓 Open source: ❌\n");
        }
        if is_honeypot == "yes" {
            audit_text += &format!("        🍯 Honeypot: ✅\n");
        } else if is_honeypot == "no" {
            audit_text += &format!("        🍯 Honeypot: ❌\n");
        }
        if is_mintable == "yes" {  
            audit_text += &format!("        🖨 Mintable: ✅\n");
        } else if is_mintable == "no" {
            audit_text += &format!("        🖨 Mintable: ❌\n");
        }   
        if is_proxy == "yes" {
            audit_text += &format!("        🔄 Proxy: ✅\n");
        } else if is_proxy == "no" {
            audit_text += &format!("        🔄 Proxy: ❌\n");
        }   
        if slippage_modifiable == "yes" {
            audit_text += &format!("        📊 Slippage modifiable: ✅\n");
        } else if slippage_modifiable == "no" {
            audit_text += &format!("        📊 Slippage modifiable: ❌\n");
        }   
        if is_blacklisted == "yes" {
            audit_text += &format!("        ⛔ Blacklisted: ❗\n");
        } else if is_blacklisted == "no" {
            audit_text += &format!("        ⛔ Blacklisted: ❌\n");
        }
        if is_contract_renounced == "yes" {
            audit_text += &format!("        📜 Contract renounced: ✅\n");
        } else if is_contract_renounced == "no" {
            audit_text += &format!("        📜 Contract renounced: ❌\n");
        }
        if is_potentially_scam == "yes" {
            audit_text += &format!("        ⚠️ Potentially scam: ❗\n");
        } else if is_potentially_scam == "no" {
            audit_text += &format!("        ⚠️ Potentially scam: ❌\n");
        }
    }

 
    let text = format!("
<a href=\"https://dexscreener.com/apechain/{token_address}\">🚀</a> {token_name}  {token_symbol}
💰 USD:  ${token_price}
💎 Mcap:  ${market_cap}
💦 Liquidity:  ${liquidity}
📈 Price history
        └ <i>1H:</i>    ${price_1h} / {variation_1h}%  
        └ <i>6H:</i>    ${price_6h} / {variation_6h}%  
        └ <i>24H:</i>  ${price_24h} / {variation_24h}% 
{audit_text}🕐 Age:  {age}
🧰 More: {social_text}
👩‍👧‍👦 Holders: {holders_count}
        └ Top 10 Holders :  ${sum_usd_amount_top_10_holders}
{holders_text} 
<code>{token_address}</code>
<a href=\"https://dexscreener.com/apechain/{token_address}\">DEX</a> <a href=\"https://apescan.io/address/{token_address}\">EXP</a>

❎ <a href=\"https://twitter.com/search?q={token_address}=typed_query&f=live\"> Search on 𝕏 </a>
📈 <a href=\"https://apescan.io/token/{token_address}\"> APE Scan </a>
");

    Ok(text)
}

fn num_floating_point(num: &f64, length: i32) -> f64 {
    ((num * 10_f64.powi(length as i32)).round()) / 10_f64.powi(length as i32)
}

fn controll_big_float(num: f64) -> String {
    if num > 1_000_000.0 {
        format!("{:.1}M", num / 1_000_000.0)
    } else if num > 1_000.0 {
        format!("{:.2}K", num / 1000.0)
    } else {
        format!("{:.3}", num)
    }
}

fn calculate_age(timestamp: &str) -> String {
    if let Ok(unix_timestamp) = timestamp.parse::<i64>() {
        let creation = DateTime::<Utc>::from_timestamp(unix_timestamp, 0).unwrap();
        let now = Utc::now();
        let duration = now.signed_duration_since(creation);
        
        let days = duration.num_days();
        if days > 365 {
            format!("{:.1} years", days as f64 / 365.0)
        } else if days > 30 {
            format!("{:.1} months", days as f64 / 30.0)
        } else {
            format!("{} days", days)
        }
    } else {
        "🔥".to_string()
    }
}


async fn get_native_token_price() -> Result<NativeToken, serde_json::Error> {
    let client = Client::new();
    let url = "https://ape.express/api/tokens/ape".to_string();

    let response = client.get(&url).send().await.unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) => Err(e),
    }
}
