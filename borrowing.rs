use std::io;
fn main() {
loop {
    println!("Enter a string to get the length of the string");
    let mut s1 = String::new(); // Create a new, mutable String

    io::stdin()
        .read_line(&mut s1) // Read input into s1
        .expect("Failed to read line");

    let len = calculate(&s1); // Pass the reference to s1 to calculate
    println!("The length of {} is {}", s1, len);
}
}
fn calculate(s: &String) -> usize {
    let length = s.len(); // Calculate length using the reference
    length
}
