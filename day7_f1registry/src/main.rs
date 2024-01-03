extern crate nickel;
extern crate serde;
extern crate serde_json;

use nickel::{Nickel, HttpRouter};
use std::collections::{BTreeMap};
use nickel::middleware;
use nickel::Serialize;
use nickel::Deserialize;
use nickel::_middleware_inner;
use nickel::as_block;

#[derive(Debug, Serialize, Deserialize)]
struct Driver {
    name: String,
    nationality: String,
}

#[derive(Debug, Clone ,Serialize, Deserialize)]
struct F1Team {
    name: String,
    country: String,
    drivers: Vec<Driver>,
}

struct F1TeamRegistry {
    teams: BTreeMap<String, F1Team>,
}

impl F1TeamRegistry {
    fn new() -> F1TeamRegistry {
        F1TeamRegistry {
            teams: BTreeMap::new(),
        }
    }

    fn register_team(&mut self, team: F1Team) -> Result<F1Team, &'static str> {
        if !self.teams.contains_key(&team.name) {
            self.teams.insert(team.name.clone(), team.clone());
            Ok(team)
        } else {
            Err("Team name already registered")
        }
    }

    fn get_all_teams(&self) -> Result<Vec<F1Team>, &'static str> {
        Ok(self.teams.values().cloned().collect())
    }
    

    fn get_team(&self, name: &str) -> Option<&F1Team> {
        self.teams.get(name)
    }
}

fn main() {
    let mut server = Nickel::new();

    let mut f1_registry = F1TeamRegistry::new();

    server.get("/teams", middleware! { |_req, _res|
        let teams = f1_registry.get_all_teams();
        serde_json::to_string(&teams).map_err(|e| (500, e))
    });

    server.post("/teams", middleware! { |request, response|
        let new_team: F1Team = request.json_as::<F1Team>().unwrap();
        match f1_registry.register_team(new_team) {
            Ok(team) => {
                serde_json::to_string(&team)
                    .map_err(|e| (500, e))
                    .and_then(|json| {
                        response.set(MediaType::Json);
                        Ok(json)
                    })
            }
            Err(err) => Err((400, err)),
        }
    });

    server.get("/teams/:name", middleware! { |request|
        let name = request.param("name").unwrap();
        if let Some(team) = f1_registry.get_team(name) {
            Ok(serde_json::to_string(team).unwrap())
        } else {
            Err((404, "Team not found"))
        }
    });

    server.listen("127.0.0.1:6767").expect("Failed to bind server");
}
