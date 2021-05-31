use super::ban_set::BanSet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bans {
    pub champion_ids: Vec<i64>,
    pub priority: f64,
}

impl Bans {
    pub fn new(ban_sets: Vec<BanSet>) -> Self {
        let mut champion_ids = Vec::new();
        let mut priority = 0.0;

        for ban_set in ban_sets {
            champion_ids.extend(ban_set.champion_ids);
            priority += ban_set.priority;
        }

        Self {
            champion_ids,
            priority,
        }
    }
}

impl Eq for Bans {}

impl Ord for Bans {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.partial_cmp(&other.priority).unwrap()
    }
}

impl PartialOrd for Bans {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl PartialEq for Bans {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
