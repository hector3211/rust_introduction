// Clap setup
use clap::Parser;
mod args;
use args::{ClapArgs,Commands};
// Anyhow 
use anyhow::Result;
// Diesel setup
mod dbstuff;
use dbstuff::{db_conn,show_entries};
use diesel::prelude::*;
// Models and Schemas
pub mod models;
pub mod schema;


fn main() ->Result<()>{
    let cli  = ClapArgs::parse();

    println!("\n");
    println!("\n");
    match &cli.command {
          Commands::New(name) => {
            println!("Employee is: {:?}",name.name);
        }
    }
    match &cli.command {
          Commands::New(invoice) => {
            println!("Invoice number #: {:?}",invoice.invoice);
        }
    }
    match &cli.command {
          Commands::New(paid) => {
            println!("is job paid? {:?}",paid.paid);
        }
    }
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    ClapArgs::command().debug_assert()
}
