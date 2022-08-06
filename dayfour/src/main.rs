struct Square {
    widht: u32,
    hiehgt: u32,
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
    let sq = Sqaure {
        width: 50,
        height: 50,
    };
    println!("area is : {}", area(sq));
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn area(square: &Sqaure) -> u32 {
    square.width * square.height
}
