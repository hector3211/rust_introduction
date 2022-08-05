fn main() {
    // tuples
    let tup = ("first in tup", 50_000);
    let (first_in_tup, second_in_tup) = tup;
    println!("{},{}", first_in_tup, second_in_tup);
    // you can de-structure them also like the following
    println!("you can also de-structure theme like {}", tup.1);
    // Lists
    let list = [200, 10, 5];
    let last_in_list = list[2];
    new_func();
    other_func(5, 1);
}

fn new_func() {
    println!("hey you called the 'new_func' function!");
}

fn other_func(x: i32, y: i32) -> i32 {
    println!("{},{} from the 'other_func'", x, y);
    let sum: i32 = x + y;
    // usually the last expression in a function is automatically returned
    return sum;
}
