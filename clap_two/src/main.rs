mod args;
use args::{
    CliArgs,
    Commands
};
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Entries {
    pub salesperson:String,
    pub invoice:i32,
    pub paid:Option<bool>,
    pub cash:Option<bool>,

}
fn main() {
    let args = CliArgs::parse();
    let mut all_entries:Vec<Entries> = Vec::new();
    // to see commands run:
    // cargo run create --help
    match args.command{
        Commands::Create(entry) => {
            println!("\n");
            print!("Created new entry ✅ {},✅ {},✅ {},✅ {}",
                entry.salesperson,
                entry.invoice,
                entry.paid.unwrap(),
                entry.cash.unwrap()
            );
            println!("\n");
            let an_entry = Entries{
                salesperson: entry.salesperson,
                invoice: entry.invoice,
                paid:entry.paid,
                cash:entry.cash,
            };
            all_entries.push(an_entry);
            println!{"Showing all entries 🟩 {:?}",all_entries};
        }
        Commands::Show => {
            println!{"Showing all entries if the application was connected to a DB ha! gotcha 💥"};
        }
    }
}
