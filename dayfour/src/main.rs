#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// here we use (Method)for our Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        username: String::from("Drama727"),
        email: String::from("drama727@yahoo.com"),
        sign_in_count: 1,
        active: true,
    };
    let name = user1.username;
    user1.username = String::from("hector");
    let user_two = build_user(String::from("maddog"), String::from("yo@email.com"));
    let user_three = User {
        username: String::from("madpepper"),
        email: String::from("yoyo@email.com"),
        ..user_two
    };
    let rect = Rectangle::square(25);
    println!("rect: {:?}", rect);
    // since rect is an instance of Reactnagle it has the method of .aera()
    println!("area is : {}", rect.area());
    let rect1 = Rectangle {
        width: 40,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 60,
    };
    println!("rect can hold react1: {}", rect.can_hold(&rect1));
    println!("react can hold rext2: {}", rect.can_hold(&rect2));
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
