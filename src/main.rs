use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // the first arg is the program name
    let config = Config::new(&args);
    println!("searching for {}, in {}", config.query, config.file_path);

    println!("In File {}", config.file_path);
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to open the file");
    println!("with content\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
