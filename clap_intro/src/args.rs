use clap::{Parser,Args,Subcommand};

#[derive(Debug,Parser)]
#[command(author, version, about )]
#[command(propagate_version = true)]
pub struct ClapArgs {
    /// First enter your name
    #[command(subcommand)]
    pub command:Commands,

}

#[derive(Subcommand,Debug)]
pub enum Commands {
    /// create a new entry
    New(NewEntry),
}

#[derive(Debug,Args)]
pub struct NewEntry {
    /// First enter your name
    #[arg(long)]
    pub name:Option<String>,
    /// second enter Invoice number
    #[arg(long,value_parser = clap::value_parser!(u32).range(1..))]
    pub invoice:Option<u32>,
    /// third is the job paid for?
    #[arg(long)]
    pub paid:Option<bool>,
}
