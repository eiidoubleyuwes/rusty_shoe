//Here I am learning slices in Rust
fn main(){
    let mut cut_me = String::from("Hello World");
    let word = first_word(&cut_me);
    cut_me.clear();
    println!("The first word is: {}", word);
}
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{ //Checks for empty space
            return &s[0..i];
        }
    }
    &s[..]
}