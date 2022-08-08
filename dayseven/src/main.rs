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
        if computer_pick == 0 {
            println!("{}", "computer picked: rock".red());
        } else if computer_pick == 1 {
            println!("{}", "computer picked: paper".red());
        } else if computer_pick == 2 {
            println!("{}", "computer picked: scissors".red());
        }
        if player_pick == 0 {
            println!("{}", "player picked: rock".blue());
        } else if player_pick == 1 {
            println!("{}", "player picked: paper".blue());
        } else if player_pick == 2 {
            println!("{}", "player picked: scissors".blue());
        }
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
