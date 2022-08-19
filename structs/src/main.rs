#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}
impl User {
    fn new_user(username: String, email: String, password: String) -> User {
        User {
            username,
            email,
            password,
        }
    }
}
#[derive(Debug)]
struct Sqft {
    width: f32,
    length: f32,
}
impl Sqft {
    // &self is a reference to the struct Sqft
    // so self has a .with and a .length param
    fn new_sqft(&self) -> f32 {
        self.width * self.length / 144 as f32
    }
}
fn main() {
    let user1 = User::new_user(
        String::from("John"),
        String::from("email@email.com"),
        String::from("password"),
    );
    println!("{:?}", user1);
    // gettting square footage
    let sqft1 = Sqft {
        width: 25.5,
        length: 101.5,
    };
    // since sqft1 is a reference to the struct Sqft
    // all we do is call the new_sqft method on sqft1
    println!("{:?}", sqft1.new_sqft());
}
