use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use serde_json::{Value};
#[derive(Deserialize,Serialize,Debug)]
struct Pokemon {
    name:Option<String>,
    url:Option<String>,
}

#[derive(Deserialize,Serialize,Debug)]
struct Results {
    results: Vec<Pokemon>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/")
        .await?
        .json::<Results>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
