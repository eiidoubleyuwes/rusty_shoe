//Understanding ownership in rust
fn main() {
    //The variable s is valid from this point forward
   {
        let s = String::from("hello");
        println!("{}", s);
   }
}
