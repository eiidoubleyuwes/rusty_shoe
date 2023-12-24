//Beer song on exercism.org Rust track
//This is my simple solution to the problem
fn main() {
    for number in (1..100).rev() {
        let numberz : u32 = number - 1;
        println!("{number} beers on the wall, {number} beers!,Take one down and pass it around {numberz} beers on the wall",);
    }
    println!("Done!");
}