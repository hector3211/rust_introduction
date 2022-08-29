// cargo run --bin show_products
extern crate diesel;
extern crate disel_three;

use self::diesel::prelude::*;
use self::disel_three::*;
use self::models::*;

fn main() {
    use disel_three::schema::products::dsl::*;

    let connection = establish_connection();
    let results = products
        .limit(5)
        .load::<Product>(&connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}", product.product_name);
        println!("----------\n");
        println!("{}", product.product_price);
    }
}
