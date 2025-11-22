use std::env;
use std::fs;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // Load .env file

    let file_path = env::var("file_path").expect("FILE_PATH not set in .env");

    println!("In file {file_path}");

    let contents = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
