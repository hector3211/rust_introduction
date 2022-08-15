use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize)]
struct Info {
    username: String,
}

// deserialize `Info` from request's body
/*
 * {
 *  "username": "<input value>"
 * }
 */
#[get("/")]
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct FormData {
    user: String,
}

#[post("/user")]
async fn user(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.user))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn data() -> impl Responder {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    HttpResponse::Ok().json(map)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(user)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/data", web::get().to(data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
