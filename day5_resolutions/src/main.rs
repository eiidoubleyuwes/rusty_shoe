//New year resolution tracker
use std::io;
use std::collections::HashMap;
fn main (){
    println!("Welcome to the New Year Resolution Tracker!");
    let mut _resolutions: HashMap<String, String>  = HashMap::new();
    let mut resolution = String::new();
    let mut status = String::new();
    loop {
    println!("Please enter a resolution:");
    io::stdin().read_line(&mut resolution).expect("Failed to read line");
    println!("Please enter the status of the resolution:");
    io::stdin().read_line(&mut status).expect("Failed to read line");
    _resolutions.insert(resolution.clone(), status.clone());
    println!("Would you like to add another resolution? (y/n)");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    if answer.trim() != "y" {
        break;
    }
}
println!("These are your resolutions and their statuses:{:?}",_resolutions);

}