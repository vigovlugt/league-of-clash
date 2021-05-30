#[derive(Clone)]
pub struct BanSet {
    pub summoner_name: String,
    pub champion_ids: Vec<i64>,
    pub priority: f64,
}

impl BanSet {
    pub fn new(summoner_name: &str, champion_ids: Vec<i64>, priority: f64) -> Self {
        Self {
            summoner_name: summoner_name.to_owned(),
            champion_ids,
            priority,
        }
    }
}

impl Eq for BanSet {}

impl Ord for BanSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.partial_cmp(&other.priority).unwrap()
    }
}

impl PartialOrd for BanSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl PartialEq for BanSet {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
