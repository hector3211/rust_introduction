// option enums
// in rust there are no NULL values!
#[derive(Debug)]
enum UsState {
    Florida,
    Alabama,
    Georgia,
    Texas,
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        /*
         here we are using the match expression to return the value of the coin
         and do something else as show un the Penny example
        */
        Coin::Penny => {
            println!("hey my penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // for match expressions we HAVE to use the NONE and the SOME
    // if we delte None we would get an error here
    // to bypass this we use _ => None;
    match x {
        // None => None;
        // since we are using an Option enum we have to wrap i + 1 in Some(i+1)
        Some(i) => Some(i + 1),
        _ => None,
    }
}
fn main() {
    let ten = Some(10);
    let eleven = plus_one(ten);
    let none = plus_one(None);
    println!("{:?}", eleven);
    let number = Some(7);
    let string = Some("hello");
    // here we use an Option enum to return a none value
    let absent: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // here we use the unwrap_or(default value:0) method to return the value if it is present or the default value
    let sum = x + y.unwrap_or(0);
    println!("{:?}", sum);
    /*
    here we call our function, passing in the value of coin
       and the value of the coin is returned
    */
    value_in_cents(Coin::Quarter(UsState::Florida));
}
