use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
enum NewsApiError {}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}
#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub async fn get_articles() -> Result<Articles, reqwest::Error> {
    dotenv().ok();
    let api_key = env::var("NEWS_API_KEY");
    let url = format!(
        "https://newsapi.org/v2/everything?q=bitcoin&apiKey={}",
        api_key.unwrap()
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    println!("{:#?}", resp.status());
    let articles: Articles = resp.json().await?;
    Ok(articles)
}
