//This program is a carry over from day8 greeting program
//For this program we will use the rand crate and it to generate a random greeting from the array
extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    let greetings = ["Hello", "olaaa", "Mkuu,sema", "Rada", "Niaje", "Habari", "Sasa", "Mambo", "Mzuri", "Vipi"];
    let mut rng = rand::thread_rng();   
    loop {
    let greeting = rng.gen_range(0..10);

        println!("Enter your name for a greeting:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Enter a valid name you bastard!");
        println!("{} {}, {}", greetings[greeting], name, "Uko fiti mkuu?");
        if name.trim() == "exit" {
            break;
        }
        let _salamu = format!("{},{},{}", greetings[greeting], name, "Uko fiti mkuu?");
       
    }

}
