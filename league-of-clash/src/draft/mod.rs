use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Draft {
    pub champion_ids: Vec<i64>,
}

impl Draft {
    pub fn new(champion_ids: Vec<i64>) -> Self {
        Draft { champion_ids }
    }
}
