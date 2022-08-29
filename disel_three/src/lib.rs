pub mod models;
pub mod schema;
#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewProduct, Product};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_product<'a>(
    conn: &PgConnection,
    product_name: &'a String,
    product_price: &'a i32,
) -> Product {
    use schema::products;

    let new_product = NewProduct {
        product_name,
        product_price,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(conn)
        .expect("Error saving new product!")
}
