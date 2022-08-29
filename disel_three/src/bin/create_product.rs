// cargo run --bin create_product
extern crate diesel;
extern crate disel_three;

use self::disel_three::*;
use std::io::stdin;
fn main() {
    let connection = establish_connection();
    println!("Please enter product name");
    let mut product_name = String::new();
    stdin().read_line(&mut product_name).unwrap();
    let product_name = &product_name[..(product_name.len() - 1)];
    println!(
        "\nNow enter a price for {} (Press {} when finished)\n",
        product_name, EOF
    );
    let mut product_price = String::new();
    stdin().read_line(&mut product_price).unwrap();
    let parsed_price = product_price.trim().parse::<i32>().unwrap();
    let product = create_new_product(&connection, &product_name.to_owned(), &parsed_price);
    println!("\nSaved product {} with id {}", product_name, product.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
