use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // the first arg is the program name
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for {}, in {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
