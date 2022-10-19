#![allow(dead_code)]
use actix_web::{get, web, HttpResponse,Responder, Result};
use serde::{Deserialize, Serialize};
pub const POKEMON_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

#[derive(Deserialize, Serialize, Debug)]
pub struct Pokemon {
    name: Option<String>,
    url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Results {
    results:Vec<Pokemon>,
}

#[get("/pokemons")]
pub async fn get_all_pokemons() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let pokemon  = reqwest::Client::new()
        .get(POKEMON_URL)
        .send()
        .await?
        .json::<Results>()            
        .await?;
    Ok(HttpResponse::Ok().json(pokemon))
}

#[derive(Deserialize,Serialize,Debug)]
pub struct SelectedPokemon{
    name:Option<String>,
}

#[get("/pokemons/{pokemon_name}")]
pub async fn get_selected_pokemon(pokemon_name: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>>{
    let pokemon = reqwest::Client::new()
        .get(format!("{}{}",POKEMON_URL,pokemon_name.to_string()))
        .send()
        .await?
        .json::<SelectedPokemon>()
        .await?;
    Ok(HttpResponse::Ok().json(pokemon))
}

#[derive(Deserialize,Serialize,Debug)]
pub struct FrontDefault {
    #[serde(rename="front_default")]
    frontdefault:String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct DreamWorld {
    #[serde(rename="dream_world")]
    dreamworld:FrontDefault,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Other {
    other:DreamWorld,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Sprites {
    sprites:Other,
}

#[get("/pokemons/picture/{name}")]
pub async fn get_pokemon_picture(name: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>>{
    let pokemon = reqwest::Client::new()
        .get(format!("{}{}",POKEMON_URL,name.to_string()))
        .send()
        .await?
        .json::<Sprites>()
        .await?;
    Ok(HttpResponse::Ok().json(pokemon))
}

