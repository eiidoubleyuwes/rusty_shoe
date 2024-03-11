//Here I am learning slices in Rust
fn main(){
    //Allow user input
    println!("Enter a string to get the first word and second word: ");
    let mut cut_me = String::new();
    std::io::stdin().read_line(&mut cut_me).unwrap();
    println!("You entered: {}", cut_me);
    let word = first_word(&cut_me);
    let maneno = second_word(&cut_me);
    println!("The first word is: {}", word);
    println!("The second word is: {}", maneno);
}
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]; //Return  a slice to the whitespace
        }
    }
    &s[..]
}
fn second_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  let mut index = 0;
  //Finds the first white space,and stores the index
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' '{
      if index == 0 {
        index = i + 1; 
      } else {
        return &s[index..i];
      }
    }
  }

  &s[..]
}