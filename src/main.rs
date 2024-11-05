pub mod token_overview;
pub mod token_info;
pub mod token_price_history;
pub mod token_top50_holders;
pub mod token_audit;

use tokio::time;
use dotenv::dotenv;
use log::{error, info};
use reqwest::Client;
use serde_json;
use std::env;
use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
use chrono::{NaiveDateTime, DateTime, Utc};
use token_overview::{TokenOverviewData, TokenOverview};
use token_info::TokenInfo;
use token_price_history::TokenPriceHistory;
use token_top50_holders::{TokenTopHolders, HolderInfo};
use token_audit::{TokenAudit, TokenAuditData, Tax};

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
    #[command(description = "Get token overview\n\tEntry type: /s ****(token address)")]
    S,
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
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd).await?;
        }
    }

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let username = msg.chat.username().unwrap();
    let message_text = msg.text().unwrap();

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, format!("Welcome to Here {username}! 🎉"))
                .await?;
        }
        Command::S => {
            let token_adr = message_text.replace("/s ", "");
            info!("Received command /s for token: {}", token_adr);
            
            let request_client = Client::new();
            let dextools_api_key = env::var("DEXTOOLS_API_KEY").expect("API_KEY not set");
            let dextools_api_plan = env::var("DEXTOOLS_API_PLAN").expect("API_PLAN not set");
            let debank_api_key = env::var("DEBANK_API_KEY").expect("API_KEY not set");

            match get_token_data(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await {
                Ok(token_data) => {
                tokio::time::sleep(time::Duration::from_secs(1)).await; //delay for 1 sec to avoid conflict request
                let token_info = get_token_info(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await.unwrap_or_default();
                tokio::time::sleep(time::Duration::from_secs(1)).await; //delay for 1 sec to avoid conflict request
                let token_price_history = get_token_price_history(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await.unwrap_or_default();
                tokio::time::sleep(time::Duration::from_secs(1)).await; //delay for 1 sec to avoid conflict request
                let token_top_holders = get_top_50_holders(request_client.clone(), &debank_api_key, &token_adr).await.unwrap_or_default();
                tokio::time::sleep(time::Duration::from_secs(1)).await; //delay for 1 sec to avoid conflict request
                let token_audit = get_token_audit(request_client.clone(), &dextools_api_key, &dextools_api_plan, &token_adr).await.unwrap_or_default();
                //make message
                let text =
                    make_token_overview_message(&token_data, &token_info, &token_price_history, &token_top_holders, &token_audit)
                        .await?;
                    bot.send_message(msg.chat.id, text)  // Changed "text" to text
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    error!("Error fetching token overview: {}", e);
                    bot.send_message(msg.chat.id, "Invalid token address")
                        .await?;
                }
            };
        }
    }
    Ok(())
}


async fn get_token_data(client: Client, api_key: &str, api_plan: &str, token_address: &str) -> Result<TokenOverviewData, serde_json::Error> {
    let url = format!(
        "https://public-api.dextools.io/{}/v2/token/{}/{}",
        api_plan, "apechain", token_address
    );

    let response = client
        .get(&url)
        .header("X-API-KEY", api_key)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str::<TokenOverview>(&text) {
        Ok(token_overview) => Ok(token_overview.data),
        Err(e) => Err(e),
    }
}

async fn get_token_info(client: Client, api_key: &str, api_plan: &str, token_address: &str) -> Result<TokenInfo, serde_json::Error> {
    let url = format!(
        "https://public-api.dextools.io/{}/v2/token/{}/{}/info",
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

async fn get_top_50_holders(
    client: Client,
    api_key: &str,
    token_address: &str,
) -> Result<TokenTopHolders, serde_json::Error> {
    let url = format!(
        "https://pro-openapi.debank.com/v1/token/top_holders?chain_id={}&id={}&start=0&limit=50",
        "ape",
        token_address
    );
    
    let response = client
        .get(&url)
        .header("AccessKey", api_key)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    
    let holders: Vec<HolderInfo> = serde_json::from_str(&text)?;
    Ok(TokenTopHolders { holders })
}


async fn make_token_overview_message(
    token_data: &TokenOverviewData,
    token_info: &TokenInfo,
    token_price_history: &TokenPriceHistory,
    token_top_holders: &TokenTopHolders,
    token_audit: &TokenAudit,
) -> Result<String, reqwest::Error> {
    //token overview
    let token_address = &token_data.address;
    let name = &token_data.name;
    let symbol = &token_data.symbol;
    let logo_url = &token_data.logo_url;
    let creation_date = &token_data.creation_date.clone().unwrap_or_default();
    let age = calculate_age(creation_date);

    //social info
    // 📊 🫧 🎨 💪 💬 🌍 🐦
    let mut social_text = String::new();
    let email = &token_data.social_info.email.clone().unwrap_or_default();
    if !email.is_empty() {
        social_text += &format!(" <a href=\"{email}\">📧 </a>");
    }
    let bitbucket = &token_data.social_info.bitbucket.clone().unwrap_or_default();
    if !bitbucket.is_empty() {
        social_text += &format!(" <a href=\"{bitbucket}\">🗃️ </a>");
    }
    let discord = &token_data.social_info.discord.clone().unwrap_or_default();
    if !discord.is_empty() {
        social_text += &format!(" <a href=\"{discord}\">💭 </a>");
    }
    let facebook = &token_data.social_info.facebook.clone().unwrap_or_default();
    if !facebook.is_empty() {
        social_text += &format!(" <a href=\"{facebook}\">ⓕ </a>");
    }
    let github = &token_data.social_info.github.clone().unwrap_or_default();
    if !github.is_empty() {
        social_text += &format!(" <a href=\"{github}\">🐱 </a>");
    }
    let instagram = &token_data.social_info.instagram.clone().unwrap_or_default();
    if !instagram.is_empty() {
        social_text += &format!(" <a href=\"{instagram}\">📸 </a>");
    }
    let linkedin = &token_data.social_info.linkedin.clone().unwrap_or_default();
    if !linkedin.is_empty() {
        social_text += &format!(" <a href=\"{linkedin}\">ℹ️ </a>");
    }
    let medium = &token_data.social_info.medium.clone().unwrap_or_default();
    if !medium.is_empty() {
        social_text += &format!(" <a href=\"{medium}\">Ⓜ️ </a>");
    }
    let reddit = &token_data.social_info.reddit.clone().unwrap_or_default();
    if !reddit.is_empty() {
        social_text += &format!(" <a href=\"{reddit}\">🎯</a>");
    }
    let telegram = &token_data.social_info.telegram.clone().unwrap_or_default();
    if !telegram.is_empty() {
        social_text += &format!(" <a href=\"{telegram}\">🕊️ </a>");
    }
    let tiktok = &token_data.social_info.tiktok.clone().unwrap_or_default();
    if !tiktok.is_empty() {
        social_text += &format!(" <a href=\"{tiktok}\">🎬 </a>");
    }
    let twitter = &token_data.social_info.twitter.clone().unwrap_or_default();
    if !twitter.is_empty() {
        social_text += &format!(" <a href=\"{twitter}\">𝕏 </a>");
    }
    let website = &token_data.social_info.website.clone().unwrap_or_default();
    if !website.is_empty() {
        social_text += &format!(" <a href=\"{website}\">🌐 </a>");
    }
    let youtube = &token_data.social_info.youtube.clone().unwrap_or_default();
    if !youtube.is_empty() {
        social_text += &format!(" <a href=\"{youtube}\">🎥</a>");
    }


    // # token Info
    // let total_supply = token_info.data.total_supply;
    // let mcap = match token_info.data.mcap {
        //     Some(cap) => cap,
        //     None => 0.0,
        // };
    let holders_count = &token_info.data.holders;
    let fdv = controll_big_float(token_info.data.fdv);

    //top price history
    let price = num_floating_point(&token_price_history.data.price, 3)  ;
    let price_1h = num_floating_point(&token_price_history.data.price_1h.unwrap_or_default(), 3);
    let price_6h = num_floating_point(&token_price_history.data.price_6h.unwrap_or_default(), 3);
    let price_24h = num_floating_point(&token_price_history.data.price_24h.unwrap_or_default(), 3);
    let variation_1h = num_floating_point(&token_price_history.data.variation_1h.unwrap_or_default(), 2);
    let variation_6h = num_floating_point(&token_price_history.data.variation_6h.unwrap_or_default(), 2);
    let variation_24h = num_floating_point(&token_price_history.data.variation_24h.unwrap_or_default(), 2);

     //top holders Info
    //  let holders_count = token_top_holders.holders.len();
     let mut sum_usd_amount_top_10_holders = 0.0;
     let mut holders_text = String::from("\n");
     let mut top_num = 0;
     let mut index_on_a_line = 0;
     let mut num_whale = 0;
     let mut num_largefish = 0;
     let mut num_bigfish = 0;
     let mut num_smallfish = 0;
     let mut num_shrimp = 0;
 
     for holder in &token_top_holders.holders {
         let holder_address = &holder.holder_address;
         let usd_amount = holder.usd_amount;
 
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
 
         let link = format!("<a href=\"https://suiscan.xyz/mainnet/account/{holder_address}?Amount={usd_amount}\">{whale_symbol}</a>");
         if index_on_a_line == 9 {
             holders_text = holders_text + &link + "\n";
             index_on_a_line = 0;
         } else {
             holders_text = holders_text + &link;
             index_on_a_line += 1;
         }
 
         if top_num == token_top_holders.holders.len() {
             holders_text += &format!("\n🐳 ( > $100K ) :  {num_whale}\n🦈 ( $50K - $100K ) :  {num_largefish}\n🐬 ( $10K - $50K ) :  {num_bigfish}\n🐟 ( $1K - $10K ) :  {num_smallfish}\n🦐 ( $0 - $1K ) :  {num_shrimp}\n");
         }
     }
    let sum_usd_amount_top_10_holders = controll_big_float(sum_usd_amount_top_10_holders);

    //token audit
    // let is_open_source = &token_audit.data.is_open_source;
    // let is_honeypot = &token_audit.data.is_honeypot;
    let is_mintable = &token_audit.data.is_mintable;
    // let is_proxy = &token_audit.data.is_proxy;
    // let slippage_modifiable = &token_audit.data.slippage_modifiable;
    let mut audit_text = String::new();
    if is_mintable == "yes" {
        audit_text += &format!("🖨 Mint: ✅");
    } else if is_mintable == "no" {
        audit_text += &format!("🖨 Mint: 🔥\n");
    }


    let text = format!("
<a href=\"https://dexscreener.com/apechain/{token_address}\">🚀</a> <a href=\"{logo_url}\">{name}  </a>{symbol}
🌐 ApeChain @ Camelot
💰 USD:  ${price}
💎 FDV:  ${fdv}
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
// Add this new function before make_token_overview_message
fn calculate_age(creation_date: &str) -> String {
    if let Ok(date) = NaiveDateTime::parse_from_str(creation_date, "%Y-%m-%dT%H:%M:%S") {
        let creation = DateTime::<Utc>::from_naive_utc_and_offset(date, Utc);
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
