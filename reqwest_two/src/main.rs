use serde::{Deserialize,Serialize};

const URL:&str = "https://pokeapi.co/api/v2/pokemon/";

#[derive(Deserialize,Serialize,Debug)]
struct Pokemon {
    name:Option<String>,
    url:Option<String>,
}

#[derive(Deserialize,Serialize,Debug)]
struct Results {
    results: Option<Vec<Pokemon>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_pokemon =  get_all().await?;
    let new_selected_pokemon = selected_pokemon("charmander".to_string()).await?;
    println!("{:#?}{:#?}",new_selected_pokemon,all_pokemon);
    Ok(())
}

async fn get_all() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/")
        .await?
        .json::<Results>()
        .await?;
    Ok(println!("{:#?}",resp))
}

#[derive(Deserialize,Serialize,Debug)]
struct Name{
    name: String,
}
async fn selected_pokemon(pokemon_name:String) -> Result<(), Box<dyn std::error::Error>>{
    let resp = reqwest::get(format!("{}{}",URL,pokemon_name))
        .await?
        .json::<Name>()
        .await?;
    Ok(println!("{:#?}",resp))
}



