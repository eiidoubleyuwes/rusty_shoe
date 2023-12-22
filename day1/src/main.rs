    //Creating a calculator app
use std::io;
fn main() {
    println!("Welcome to simple calculator,it can only however do two numbers at a time");
    println!("Enter the first number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Number expected");
    //Convert num1 to float and trim the enter key
    let num1: f32 = num1.trim().parse().expect("Number expected");
    println!("Enter your operator(+,-,*,/,%)");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Operator expected");
    let operator: char = operator.trim().parse().expect("Operator expected");
    println!("Enter the second number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Number expected");
    //Convert num2 to float and trim the enter key
    let num2: f32 = num2.trim().parse().expect("Number expected");
    match operator {
        '+' => println!("{}", num1 + num2),
        '-' => println!("{}", num1 - num2),
        '*' => println!("{}", num1 * num2),
        '/' => println!("{}", num1 / num2),
        '%' => println!("{}", num1 % num2),
        _ => println!("Invalid operator"),
    }
}
