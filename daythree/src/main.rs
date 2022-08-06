fn main() {
    // tuples
    let tup = ("first in tup", 50_000);
    let (first_in_tup, second_in_tup) = tup;
    println!("{},{}", first_in_tup, second_in_tup);
    // you can de-structure them also like the following
    println!("you can also de-structure theme like {}", tup.1);
    // Arrays and going through arrays
    let list = [200, 10, 5];
    let last_in_list = list[2];
    println!("{}", last_in_list);
    let sum = other_func(10, 1);
    println!("sum: {}", sum);
    // four loop
    for item in list.iter() {
        println!("items in list are: {}", item)
    }
    // four loop going though a range
    for number in 1..10 {
        println!("{}", number);
    }
}

fn other_func(x: i32, y: i32) -> i32 {
    // usually the last expression in a function is automatically returned
    x + y
}
