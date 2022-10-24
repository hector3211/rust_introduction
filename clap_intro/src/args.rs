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
    New(EntryCommand),
}

#[derive(Debug,Args)]
pub struct EntryCommand {
    #[command(subcommand)]
    pub command: EntrySubcommands,
}

#[derive(Debug,Subcommand)]
pub enum EntrySubcommands {
    Create(CreateEntry),
    // Update(UpdateEntry),
    Show,

}

#[derive(Debug,Args)]
pub struct CreateEntry {
    /// First enter your name
    #[arg(long)]
    pub name:String,
    /// second enter Invoice number
    #[arg(long,value_parser = clap::value_parser!(i32).range(1..))]
    pub invoice:i32,
    /// third is the job paid for?
    #[arg(long)]
    pub paid:bool,
}
#[derive(Debug,Args)]
pub struct UpdateEntry {
    /// Update paid feild
    #[arg(long)]
    pub paid:bool,
}
