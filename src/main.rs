use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    println!("Searching for {} in {}", query, filename);

    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file!");
}

struct Config {
    query: String,
    file: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file = args[2].clone();

    Config { query, file }
}
