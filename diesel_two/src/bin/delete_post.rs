// use cargo run --bin delete_post <title of post>
extern crate diesel;
extern crate diesel_two;

use self::diesel::prelude::*;
use self::diesel_two::*;
use std::env::args;

fn main() {
    use diesel_two::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
