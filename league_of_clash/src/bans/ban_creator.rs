use std::collections::BinaryHeap;

use crate::champion_stats::champion_stats::ChampionStats;

use super::{ban_set::BanSet};

pub struct BanCreator {
    pub best_bans: BinaryHeap<BanSet>
}

impl BanCreator {
    pub fn new() -> Self {
        Self {
            best_bans: BinaryHeap::<BanSet>::new()
        }
    }

    pub fn add_summoner_stats(&mut self, summoner_name: &str, stats: &Vec<ChampionStats>) {
        let champion_size = stats.iter().filter(|x| x.score > 0.0).count().clamp(0, 3);

        for i in 0..champion_size {
            let champion_ids = (0..=i).map(|i| stats[i].champion_id).collect::<Vec<_>>();

            let next_score = if i == champion_size - 1 {
                0.0
            }else {
                stats[i+1].score
            };

            let priority = stats[0].score - next_score ; // / (i + 1) as f64

            self.best_bans.push(BanSet::new(summoner_name, champion_ids, priority))
        }
    }

    pub fn get_best_bans(self) -> Vec<BanSet> {
        let mut bans = self.best_bans.into_sorted_vec();
        bans.reverse();
        bans
    }
}