use std::env;
use std::process;

use chapter12::{self, CommandConfig};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = CommandConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = chapter12::run(config) {
        eprintln!("application error: {err}");
        process::exit(1);
    }
}