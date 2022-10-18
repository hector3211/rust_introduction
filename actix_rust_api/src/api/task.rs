use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
}
// add to database
#[get("/json/{name}")]
pub async fn json_data(name: web::Path<String>) -> Result<impl Responder> {
    let data = User {
        name: name.to_string(),
    };
    Ok(web::Json(data))
}

// deserialize `Info` from request's body
/*
 * {
 *  "username": "<input value>"
 * }
 * so its http://127.0.0.1:8080/username
 */
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello this is index")
}

#[derive(Deserialize)]
pub struct FormData {
    user: String,
}

// port: 8080/hector
#[post("/user")]
pub async fn user(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.user))
}

// here we GET our "data"
pub async fn data() -> impl Responder {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    HttpResponse::Ok().json(map)
}
