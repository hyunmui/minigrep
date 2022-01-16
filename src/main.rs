use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("search word: {}", config.query);
    println!("target file name: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Can't read this file");

    println!("File Content: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
