//Learning how to declare constants in Rust
//This simple program converts minutes to hours and seconds
use std::io;
fn main() {
    const SECONDS_PER_MINUTE: u32 = 60;
    const MINUTES_PER_HOUR: u32 = 60;
    println!("Enter the number of minutes: ");
    let mut minutes = String::new();
    io::stdin().read_line(&mut minutes)
        .expect("Enter valid minutes");
    let minutes: u32 = minutes.trim().parse()
        .expect("Enter valid minutes");
    let hours = minutes / MINUTES_PER_HOUR;
    let seconds = minutes * SECONDS_PER_MINUTE;
    println!("{} minutes is {} hours and {} seconds", minutes, hours, seconds);
}
