#[macro_use]
extern crate diesel;
extern crate dotenvy;
// mods
mod args;
mod db;
mod models;
mod schema;
mod controllers;
// Clap setup
use clap::Parser;
use args::{CliArgs,Commands};
// Anyhow 
use anyhow::Result;
// controllers
use crate::controllers::show_entries::{
    handle_entry_command,
    handle_show_command
};

fn main() ->Result<()>{
    let cli  = CliArgs::parse();

    match cli.command {
        Commands::New(entry) => handle_entry_command(entry),
        Commands::Show => handle_show_command()
    }
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert()
}
