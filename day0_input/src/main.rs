use std::io;
fn main() {
    println!("Hello,traveller,please input your name:");
    let mut user_name  = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    println!("Hello,{}", user_name);
}
