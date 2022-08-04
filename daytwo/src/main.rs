use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess a number!");
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("something went wrong!");
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too high".red()),
            Ordering::Equal => {
                println!("{}","you won!".green());
                break;
            }
        }
    }
}
