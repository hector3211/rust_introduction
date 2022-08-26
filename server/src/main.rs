use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
mod api;
mod repository;

use api::task::get_task;
use repository::pdb::establish_connection;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(establish_connection)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
