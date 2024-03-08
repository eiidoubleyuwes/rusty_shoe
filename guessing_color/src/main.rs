use rand::Rng;
use std::io;
use colored::*;
use std::cmp::Ordering;
fn main() {
   println!("Hello, welcome to the guesssing game");
   println!("Please enter a number");
   let mut guess = String::new();
   io::stdin()
   .read_line(&mut guess)
   .expect("Failed to read line");
   println!("You guessed : {}" , guess);
   let guess: u32 = guess.trim().parse().expect("Not a number");
   let secret = rand::thread_rng().gen_range(1..1001);
   println!("The number was: {}" , secret);
   match guess.cmp(&secret) {
	Ordering::Less => println!("{}","You lose,too low" .red()),
	Ordering::Greater =>println!("{}","You lose,too high" .red()),
	Ordering::Equal => println!("{}","You won!" .green()),
}
}

