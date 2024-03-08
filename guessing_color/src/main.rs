use rand::Rng;
use std::io;
use colored::*;
use std::cmp::Ordering;
fn main() {
   println!("Hello, welcome to the guesssing game");
   loop{
   let secret = rand::thread_rng().gen_range(1..1001);
   println!("Please enter a number");
   println!("The random number is: {}" , secret);
   let mut guess = String::new();
   io::stdin()
   .read_line(&mut guess)
   .expect("Failed to read line");
   println!("You guessed : {}" , guess);
   let guess: u32 = match guess.trim().parse() {
	Ok(num) =>num,
	Err(_) => continue};
   match guess.cmp(&secret) {
	Ordering::Less => println!("{}","You lose,too low" .red()),
	Ordering::Greater =>println!("{}","You lose,too high" .red()),
	Ordering::Equal => {println!("{}","You won!" .green()); break;},
}
}
}

