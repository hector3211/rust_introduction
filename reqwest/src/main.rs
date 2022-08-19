use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Debug, Deserialize)]
struct Rating {
    id: i32,
    title: String,
    rating: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("RATINGS_API_KEY")?;
    let api_token = env::var("RATINGS_API_TOKEN")?;
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))?,
    );
    headers.insert("apikey", api_key.parse().unwrap());
    let client = reqwest::Client::new();
    let resp = client
        .get("https://xguojyfzdarlnkfaruty.supabase.co/rest/v1/ratings?id=eq.3&select=*")
        .headers(headers)
        .send()
        .await?;
    println!("{:#?}", resp);
    let resp_json = resp.json::<Vec<Rating>>().await?;
    println!("{:#?}", resp_json);
    Ok(())
}
