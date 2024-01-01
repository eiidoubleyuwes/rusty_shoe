//Learning about hashmaps
use std::collections::HashMap;
fn main() {
    let mut f1_teams = HashMap::new();
    f1_teams.insert("Mercedes", "Red Bull");
    f1_teams.insert("Ferrari", "McLaren");
    f1_teams.insert("Aston Martin", "Alpine");
    f1_teams.insert("AlphaTauri", "Alfa Romeo"); 
    f1_teams.insert("Haas", "Williams");
    println!("{:?}", f1_teams);
    println!("The number of teams in F1 is {}", f1_teams.len());
   
}
