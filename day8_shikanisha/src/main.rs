//This program takes a user's name and creates a custom swahili greeting
use std::io;
fn main() {
    println!("Enter your name for a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Enter a valid name you bastard!");
    let salamu = "Habari yako!";
    let checkin = "Uko fiti mkuu?";
    let greeting = format!("{},{},{}", salamu, name, checkin);
    println!("{}", greeting)
}
