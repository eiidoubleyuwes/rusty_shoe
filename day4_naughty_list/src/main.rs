//This program allows user input and checks whether you are on the naughty list
//Merry Christmas!

use std::io;
fn main() {
    let mut name = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}!", name);
    //loop that puts all the names apart from Baraka in the good list
    let mut naughty_list = String::new();
    for i in name.split(" ") {
        if i == "Baraka" {
            naughty_list.push_str(i);
        }
        else {
            naughty_list.push_str(" ");
            naughty_list.push_str(i);
        }
}
    //checks if you are on the naughty list
    if naughty_list.contains("Baraka") {
        println!("You are on the naughty list!");
    }
    else {
        println!("You are not on the naughty list!");
    }
    println!("{}", naughty_list);
}
