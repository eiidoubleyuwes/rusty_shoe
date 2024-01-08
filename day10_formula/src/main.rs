//Simple program using serde to read data from a JSON file
//The JSON file is a list of formula 1 drivers and their stats from 2022
//The use should be able to have an input of a driver's name and the program will return the driver's stats
use serde_json;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
extern crate serde;
#[derive(Debug, Deserialize, Serialize)]
struct Formula1Driver {
    driver: String,
    team: String,
    age: u32,
    wins: u32,
    most_overtakes: u32,
    poles: u32,
    laps_led: u32,
    // Add other relevant fields as needed
}

fn main() {
    
    let path = "./src/csvjson.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let drivers: Vec<Formula1Driver> = serde_json::from_str(&data).expect("Unable to parse");

    println!("Enter a driver's name(First and Last):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Find the driver in the vector based on the entered name
    if let Some(driver) = drivers.iter().find(|&d| d.driver == input) {
        //Deserialise to a string
        let json_string = serde_json::to_string(driver).unwrap();
        println!("The stats for {} are: {}", driver.driver, json_string);
       
    } else {
        println!("Driver not found");
    }
    
}