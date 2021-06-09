use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub players: Vec<String>,
    pub region: String,
}

impl Team {
    pub fn new(players: Vec<String>, region: String) -> Self {
        Team { players, region }
    }
}
