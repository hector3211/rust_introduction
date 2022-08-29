#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}
fn print_struct(a_struct: &User) {
    println!("{:#?}", a_struct);
}
fn main() {
    let new_user = User {
        id: 1,
        name: "madDog".to_owned(),
        email: "madDog@email.com".to_owned(),
    };
    print_struct(&new_user)
}
