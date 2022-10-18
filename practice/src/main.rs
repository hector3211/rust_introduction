use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}
fn print_struct() -> User {
    let new_user_struct = User {
        id: 2,
        name: "hello".to_string(),
        email: "email.com".to_string(),
    };
    return new_user_struct;
}
fn parsed_json() -> Result<()> {
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
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn parsed_json_into_struct() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {},his age is: {}", p.name, p.age);
    Ok(())
}

fn main() {
    println!("{:?}", print_struct());
    println!("{:?}", parsed_json());
    println!("{:?}", parsed_json_into_struct());
}
