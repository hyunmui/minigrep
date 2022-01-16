use std::{env, process};

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        print!("An error occurred while parsing the argument: {}", err);
        process::exit(1);
    });

    println!("search word: {}", config.query);
    println!("target file name: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
