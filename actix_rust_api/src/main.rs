use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder};
mod api;
use api::pokemon::{get_all_pokemons, get_selected_pokemon, get_pokemon_picture};
use api::task::{data, index, json_data, user};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(json_data)
            .service(user)
            .service(echo)
            .service(get_all_pokemons)
            .service(get_selected_pokemon)
            .service(get_pokemon_picture)
            .route("/hey", web::get().to(manual_hello))
            .route("/data", web::get().to(data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
