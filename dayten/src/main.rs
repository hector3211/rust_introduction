// Generic types

// heres  generic struct
// T is one type
// U is another type
struct Point<T, U> {
    x: T,
    y: U,
}
// heres  generic enums
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
// in the Result we actually use two generics T and E
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// A generic implementation for our Point struct (like and class and methods)
// methods being inside the impl
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}
// we state this function takes generic types with <T> for types
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let v1 = vec![1, 2, 3];
    let largest_number = get_largest(v1);
    println!("The largest number is {}", largest_number);
    let p1 = Point { x: 5.5, y: 10.6 };
    let p2 = Point {
        x: "Hello",
        y: "hey",
    };
    // here we call our impl of fn x
    p2.x();
    println!("{:?}", p2.y());
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    // here we call our option enum
    let option_enum1 = Option::Some(5);
    let option_enum2 = Option::Some(5.9);
    println!("{:?}", option_enum1);
    println!("{:?}", option_enum2);
}
