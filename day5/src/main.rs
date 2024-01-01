//Learning about hashmaps
use std::collections::HashMap;
fn main() {
    let mut f1_teams = HashMap::new();
    f1_teams.insert("Mercedes", "Red Bull", "Ferrari", "McLaren", "Aston Martin", "Alpine", "AlphaTauri", "Alfa Romeo", "Haas", "Williams");
    println!("The F1 teams are: {}", f1_teams);
}
