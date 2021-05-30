use super::ban_set::BanSet;

pub struct Bans {
    pub ban_sets: Vec<BanSet>,
    pub priority: f64,
}

impl Bans {
    pub fn new(ban_sets: Vec<BanSet>) -> Self {
        let priority = ban_sets.iter().map(|x| x.priority).fold(0.0, |a, b| a + b);
        Self { ban_sets, priority }
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
