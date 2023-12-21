fn main() {
    // let x = 4;
    // let y = 10;
    // println!("x = {}, y = {}", x, y);
    //tuples in Rust are cool also they are immutable 
    let tup = (1,4,'z',false,"yoh");
    println!("{}",tup.0);
    // time to make a mutable one using mut key word
    let mut tup2 = (1,4,'z',false,"yoh");
    tup2.0 = 78;
    println!("{}",tup2.0);


}
