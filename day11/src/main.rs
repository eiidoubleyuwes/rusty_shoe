//Today I am taking on the popular FizzBuzz challenge.
//The challenge is to print out the numbers from 1 to 100, but for multiples of three print "Fizz" instead of the number and for the multiples of five print "Buzz". For numbers which are multiples of both three and five print "FizzBuzz".
//I will be using Rust for this challenge.
use std::io;
fn main() {
    println!("Welcome to FizzBuzz!");
    println!("Please enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    for i in 1..input {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
}
