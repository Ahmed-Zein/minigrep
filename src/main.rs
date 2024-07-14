use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // the first arg is the program name
    let query = &args[1];
    let file_path = &args[2];
    println!("searching for {query}, in {file_path}");

    println!("In File {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to open the file");
    println!("with content\n{contents}");
    
}
