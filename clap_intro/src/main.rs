use clap::Parser;
mod args;
use args::ClapArgs;


fn main() {
    let args  = ClapArgs::parse();

    println!("name: {:?}",args.name);
    println!("invoice number: {:?}",args.invoice_number);
    println!("paid for: {:?}",args.paid_for)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    ClapArgs::command().debug_assert()
}
