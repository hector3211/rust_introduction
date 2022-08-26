use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Deserializing {} , {}", v["name"], v["phones"][0]);
    let v2 = serde_json::to_string(&v)?;
    println!("Serializing :{:#?}", v2);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "typed_example",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.

    println!("{:#?} {:#?}", p.age, p.name);
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}
impl User {
    fn new_user(id: u32, name: String, email: String) -> User {
        User { id, name, email }
    }
}

fn func(creds: User) -> Vec<User> {
    let user = User::new_user(creds.id, creds.name, creds.email);
    let mut user_vec = Vec::new();
    user_vec.push(user);
    user_vec
}

fn main() {
    let user1 = User {
        id: 1,
        name: "hector".to_owned(),
        email: "hector@email.com".to_owned(),
    };
    println!("{:#?}", user1);
    let user2 = User::new_user(2, "drama".to_owned(), "email@emailcom".to_owned());
    println!("{:#?}", &user2);
    println!("{:#?}", func(user1));
    println!("{:#?}", typed_example());
    println!("{:#?}", untyped_example())
}
