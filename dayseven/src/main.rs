use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// Rust Rock Paper Scissors Game
fn main() {
    println!("welcome to rock paper scissors");
    loop {
        let computer_pick = rand::thread_rng().gen_range(0, 3);
        let mut player_pick = String::new();
        println!("{}", "welcome to Rock Paper Scissors".yellow());
        println!("{}", "0: rock 1: paper 2: scissors 3: quit".yellow());
        io::stdin()
            .read_line(&mut player_pick)
            .expect("failed to read pick");
        let player_pick: u8 = player_pick.trim().parse().expect("please type a number");
        if player_pick == 3 {
            println!("{}", "Goodbye".red());
            break;
        }
        get_pick(computer_pick, "Computer".to_string());
        get_pick(player_pick, "player".to_string());
        match player_pick.cmp(&computer_pick) {
            Ordering::Less => println!("{}", "you lose".red()),
            Ordering::Greater => {
                println!("{}", "you win".green());
                break;
            }
            Ordering::Equal => println!("{}", "draw".yellow()),
        }
    }
}
fn get_pick(number: u8, person: String) {
    match number {
        0 => println!("{} picked: {}", person, "rock"),
        1 => println!("{} picked: {}", person, "paper"),
        2 => println!("{} picked: {}", person, "scissors"),
        _ => println!("{} picked: {}", person, "invalid pick"),
    }
}
