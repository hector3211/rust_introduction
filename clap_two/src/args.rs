use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug,Subcommand)]
pub enum Commands {
    Create(CreateCommand),
    Show,
}

#[derive(Debug,Args)]
pub struct CreateCommand {
    /// name of sales person
    #[arg(short,long)]
    pub salesperson:String,
    /// Invoice number for the job
    #[arg(short,long,value_parser = clap::value_parser!(i32).range(1..))]
    pub invoice:i32,
    /// is the job paid for?
    #[arg(short,long)]
    pub paid:Option<bool>, 
    // for right now bool args only show up when has an Option
    /// was it crash?
    #[arg(short,long)]
    pub cash:Option<bool>,
    // for right now bool args only show up when has an Option

}
