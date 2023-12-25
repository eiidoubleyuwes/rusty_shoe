//Today we are learning about accepting user input because next we are making a text editor
use std::io;
use std::io::Read;
fn main() {
   //for loop to read characters from the keyboard
   //This program will just print out the characters you put in until you hit ctrl+c to stop the run

    for b in io::stdin().bytes() {            
        let c = b.unwrap() as char;            
        println!("{}", c);                          
        }
}
