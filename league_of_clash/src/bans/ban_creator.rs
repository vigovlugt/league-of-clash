use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

use crate::champion_stats::champion_stats::ChampionStats;

use super::{ban_set::BanSet, bans::Bans};

pub struct BanCreator {
    pub best_bans: BinaryHeap<BanSet>,
}

impl BanCreator {
    pub fn new() -> Self {
        Self {
            best_bans: BinaryHeap::<BanSet>::new(),
        }
    }

    pub fn add_summoner_stats(&mut self, summoner_name: &str, stats: &Vec<ChampionStats>) {
        let champion_size = stats.iter().filter(|x| x.score > 0.0).count();
        let top_3 = champion_size.clamp(0, 3);

        for i in 0..top_3 {
            let champion_ids = (0..=i).map(|i| stats[i].champion_id).collect::<Vec<_>>();

            let next_score = if i == champion_size - 1 {
                0.0
            } else {
                stats[i + 1].score
            };

            let priority = stats[0].score - next_score; // / (i + 1) as f64

            self.best_bans
                .push(BanSet::new(summoner_name, champion_ids, priority))
        }
    }

    fn get_bansets_by_size(ban_sets: Vec<BanSet>) -> HashMap<usize, Vec<BanSet>> {
        let mut hashmap = HashMap::new();
        for ban_set in ban_sets.into_iter() {
            let champion_size = ban_set.champion_ids.iter().count();
            hashmap
                .entry(champion_size)
                .or_insert(Vec::new())
                .push(ban_set);
        }

        hashmap
    }

    fn get_all_bans(hashmap: &mut HashMap<usize, Vec<BanSet>>) -> Vec<Bans> {
        let mut bans = Vec::new();

        if let Some(size_3_bans) = hashmap.remove(&3) {
            for ban_set in size_3_bans {
                bans.push(Bans::new(vec![ban_set]))
            }
        }

        if let Some(size_2_bans) = hashmap.remove(&2) {
            if let Some(size_1_bans) = hashmap.remove(&1) {
                let product = size_2_bans
                    .into_iter()
                    .cartesian_product(size_1_bans.into_iter());

                let new_bans = product.map(|(size_1, size_2)| Bans::new(vec![size_2, size_1]));

                bans.extend(new_bans);
            }
        }

        bans
    }

    pub fn get_best_bans(self) -> Vec<Bans> {
        let bans = self.best_bans.into_vec();

        let mut hashmap = BanCreator::get_bansets_by_size(bans);

        let mut all_bans = BanCreator::get_all_bans(&mut hashmap);

        all_bans.sort();
        all_bans.reverse();

        all_bans
    }
}
