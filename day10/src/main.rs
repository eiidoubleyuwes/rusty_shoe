//Learning how to read from a file and parse JSON data
use serde_json;
use std::fs;

fn main() {
    let path = "./src/input.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("{}", res);
}