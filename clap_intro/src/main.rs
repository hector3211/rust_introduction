// Clap setup
use clap::Parser;
mod args;
use args::{ClapArgs,Commands,NewEntry};
// Anyhow 
use anyhow::Result;
// Diesel setup
use diesel::pg::PgConnection;
use diesel::prelude::*;
// ENV setup
use dotenvy::dotenv;
use std::env;
// Models and Schemas
pub mod models;
use self::models::*;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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
