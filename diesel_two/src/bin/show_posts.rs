// to show posts
// cargo rin --bin show_posts
extern crate diesel;
extern crate diesel_two;

use self::diesel::prelude::*;
use diesel_two::models::Post;
use diesel_two::*;
fn main() {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
