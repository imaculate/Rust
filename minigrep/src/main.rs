
use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // return value is (), no need to unwrap here
    if let Err(e) = minigrep::run(config)
    {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}