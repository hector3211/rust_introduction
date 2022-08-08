use std::collections::HashMap;
fn main() {
    // Vectors are growable arrays
    // if no values are provided, the vector is empty
    // which we need to provided the type of the vector example: Vec<i32>
    // to add values to it, it needs to be mutable
    let mut v: Vec<i32> = Vec::new();
    // pushing to our vector
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    {
        // creating a vector with values using vec! macro
        let v2 = vec![4, 5, 6];
        println!("{:?}", v2);
        // this can only be called in this scope
        // indexing in a vector is done with the [index] syntax
        let number_six = &v2[2];
        println!("{:?}", number_six);
        // if the index is out of bounds, it will return a error
        //example: let number_seven = &v2[20];
        // so we use the get method to return a Option<T>
        match v2.get(20) {
            Some(five) => println!("{:?}", five),
            // since we want index 20 it'll return None
            None => println!("No number at index 20"),
        }
    }
    // iterating over a vector
    let mut v3 = vec![7, 8, 9, 10];
    // four loop to iterate over a vector
    for number in &mut v3 {
        // to change the value of a number in a vector
        // we use *number
        *number += 10;
        println!("{:?}", number);
    }
    {
        // Strings and changing or adding the value of a string
        let mut s = String::from("hello");
        s.push_str(" world");
        // for charactors use '' not ""
        s.push('!');
        println!("{:?}", s);
        // format macro
        let s1 = String::from("blue");
        let s2 = String::from("red");
        let s3 = format!("{} {}", s1, s2);
        println!("{:?}", s3);
        // indexing in a string
        let hello = String::from("hello");
        let h = &hello[0..1];
        println!("{:?}", h);
    }
    {
        // Hsshmaps like disctionaries in python
        // first we import it
        // use std::collections::HashMap;
        let yellow = String::from("yellow");
        let green = String::from("green");
        // creating hashmap
        let mut scores = HashMap::new();
        // adding values to the hashmap
        scores.insert(yellow, 10);
        scores.insert(green, 20);
        // getting values from the hashmap
        let team_yellow = String::from("yellow");
        // we pass the key to the get value
        let score = scores.get(&team_yellow);
        println!("{:?}", score);
        // iterating over our hashmap using a four loop
        for (key, value) in &scores {
            println!("{:?}:{:?}", key, value);
        }
        // updating values in the hashmap
        let text = "Hello new world you earthlings have a wondeful world";
        // creating a hashmap
        let mut map = HashMap::new();
        // .split_whitespace() is a method that splits a string into a vector of strings
        for word in text.split_whitespace() {
            /*
            our for loop will iterate over each word in the text
            entry a word in our hashmap
            if the word is already in the hashmap, we increment the value by 1
            if the word is not in the hashmap using or_insert(0),
            we add the word and set the value to 1
             */
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}
