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
    /// Create a new entry
    New(EntryCommand),
    /// Update a entyr
    // Update(UpdateEntry),
    /// Show all entries
    Show,
}

#[derive(Debug,Args)]
pub struct EntryCommand {
    #[command(subcommand)]
    pub command: EntrySubcommands,
}

#[derive(Debug,Subcommand)]
pub enum EntrySubcommands {
    Create(CreateEntry),
}

#[derive(Debug,Args)]
pub struct CreateEntry {
    #[arg(short,long)]
    pub name:String,
    #[arg(short,long,value_parser = clap::value_parser!(i32).range(1..))]
    pub invoice:i32,
    #[arg(short,long)]
    pub paid:Option<bool>,
}

#[derive(Debug,Args)]
pub struct UpdateEntry {
    #[arg(long)]
    pub id:i32,
    #[arg(long)]
    pub name:String,
    #[arg(long,value_parser = clap::value_parser!(i32).range(1..))]
    pub invoice:i32,
    #[arg(long)]
    pub paid:bool,
}
