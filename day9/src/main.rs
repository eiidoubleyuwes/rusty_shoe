//This is a refresh of a previous program that involved using random. This time we will use the rand crate
//This is also a refresher on using arrays and other data stores in rust
//It also prints a random team from the array
extern crate rand;
use rand::Rng;
fn main() {
   let f1_teams: [&str; 10]  = ["Mercedes", "Red Bull", "Ferrari", "McLaren", "Aston Martin", "Alpine", "AlphaTauri", "Alfa Romeo", "Haas", "Williams"];
   let mut rng = rand::thread_rng();
//To print out a random team from the array
   let f1_team = rng.gen_range(0..10);
   for i in 0..10 {
    println!("The team {} is {}", i, f1_teams[i]);
   println!("The random team is {}", f1_teams[f1_team]); 
   }
}
