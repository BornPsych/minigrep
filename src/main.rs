use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path  = &args[2];
    println!("Searching the {query} and file path {file_path}");
    // dbg!(args);
    let contents = fs::read_to_string(file_path).expect("he file path you are expecting is missing");

    println!("the file contains the folloowing content {}", contents)
}