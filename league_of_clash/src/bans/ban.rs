pub struct Ban {
    pub summoner_name: String,
    pub champion_id: i64,
    pub priority: f64,
}

impl Ban {
    pub fn new(summoner_name: &str, champion_id: i64, priority: f64) -> Self {
        Self {
            summoner_name: summoner_name.to_owned(),
            champion_id,
            priority,
        }
    }
}

impl Eq for Ban {}

impl Ord for Ban {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.partial_cmp(&other.priority).unwrap()
    }
}

impl PartialOrd for Ban {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl PartialEq for Ban {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
