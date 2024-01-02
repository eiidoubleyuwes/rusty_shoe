//Simple inventory management program for an f1 team
struct pit_crew{
    name: String,
    team: String,
    points: u32,
}

fn get_pit_crew(name: String, team: String, points: u32) -> pit_crew{
    pit_crew{
        name,
        team,
        points,
    }
}
fn add_pit_crew(pit_crew: &mut Vec<pit_crew>, name: String, team: String, points: u32){
    pit_crew.push(get_pit_crew(name, team, points)); //move to next owner
}
