use clap::Parser;

#[derive(Debug,Parser)]
#[command(author, version, about )]
pub struct ClapArgs {
    /// First enter your name
    #[arg(short,long)]
    pub name:Option<String>,
    /// second enter Invoice number
    #[arg(short,long,value_parser = clap::value_parser!(u32).range(1..))]
    pub invoice_number:Option<u32>,
    /// third is the job paid for? true or false
    #[arg(short,long)]
    pub paid_for:Option<bool>,

}
