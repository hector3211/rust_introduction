// to publish a post
// cargo run --bin publish_post <id of post>
extern crate diesel;
extern crate diesel_two;

use self::diesel::prelude::*;
use self::diesel_two::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_two::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
