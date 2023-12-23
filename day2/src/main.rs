//Learning how to use random numbers in Rust
extern crate rand;
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..100);
    println!("Random number: {}", random_number);
}

