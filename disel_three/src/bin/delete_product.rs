// cargo run --bin delete_product <name of product>
extern crate diesel;
extern crate disel_three;

use self::diesel::prelude::*;
use self::disel_three::*;
use std::env::args;

fn main() {
    use disel_three::schema::products::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(products.filter(product_name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting product");

    println!("Deleted {} product", num_deleted);
}
