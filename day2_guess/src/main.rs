//A game in rust that forces the user to guess a number between one and 2.
//The user is up againgst the computer.
//The user has a chance to choose a character although as of now it does nothing.
//If the user guesses the number correctly, the game ends.
extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    println!("Hello,Traveller,please chose a character between 1.Mage,2.Warrior,3.Rogue");
    let mut avatar = String::new();
    io::stdin().read_line(&mut avatar).expect("Pleace choose a value between 1 and 3");
    let avatar: u32 = avatar.trim().parse().expect("Please type a number!");
    match avatar {
        1 => println!("You have chosen the Mage"),
        2 => println!("You have chosen the Warrior"),
        3 => println!("You have chosen the Rogue"),
        _ => println!("You have chosen the wrong character"),
    }
    println!("A beast has appeared,please choose a number between 1 and 2");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Please type a number!");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let mut rng = rand::thread_rng();
    let beast=rng.gen_range(1..3);
    match guess.cmp(&beast) {
        std::cmp::Ordering::Less => println!("Wrong!Too small!"),
        std::cmp::Ordering::Greater => println!("Wrong,Too big!"),
        std::cmp::Ordering::Equal => println!("You win!"),
    }
    println!("The secret number is: {} and your guess was {}", beast,guess);
}
