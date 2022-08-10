use std::collections::HashMap;
#[derive(Debug)]
enum Array {
    Int(i32),
    Str(String),
}
fn main() {
    // we create a mutable hashmap
    let mut review_store = HashMap::new();
    // insert our first piece of data
    review_store.insert(String::from("The Shining"), 5);
    println!("{:?}", review_store);
    // insert our second piece of data
    add_review(&mut review_store, "Captain marvel".to_string(), 5);
    println!("{:?}", review_store);
    let mut vector = Vec::new();
    vector.push(1);
    let vector_with_multiple_types = vec![Array::Int(1), Array::Str(String::from("hello"))];
    println!("{:?}", vector_with_multiple_types);
    for item in vector_with_multiple_types {
        match item {
            Array::Int(i) => println!("{}", i),
            Array::Str(s) => println!("{}", s),
        }
    }
}
/*
 * add_review takes a reference to our review_store
 * a movie title thats a String
 * and a rating thats an integer
 * and adds the movie and rating to the review_store
 */
fn add_review(review_store: &mut HashMap<String, i32>, movie: String, rating: i32) {
    review_store.insert(movie, rating);
}
