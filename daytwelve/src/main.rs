// CLI App
use daytwelve::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    daytwelve::run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}
