fn main () {
    let s1 : String = String::from("Hello");
    let len = calculate(&s1);
    println!("the length of {} is {}" ,s1 ,len);
}
fn calculate (s: &String) -> usize {
    let length = s.len();
    length
}
