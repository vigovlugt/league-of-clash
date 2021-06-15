pub mod champion_role_player_prediction;
pub mod draft_prediction;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use itertools::Itertools;
use league_of_clash_shared::{dataset::Dataset, role::Role};

use crate::{draft::Draft, player_stats::PlayerStats, team::Team};

use self::{
    champion_role_player_prediction::ChampionRolePlayerPrediction,
    draft_prediction::DraftPrediction,
};

#[derive(Serialize, Deserialize)]
pub struct ChampionRolePlayerPredictor {
    dataset: Dataset,
    player_stats: HashMap<String, PlayerStats>,
}

impl ChampionRolePlayerPredictor {
    pub fn new(
        dataset: Dataset,
        player_stats: HashMap<String, PlayerStats>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ChampionRolePlayerPredictor {
            dataset,
            player_stats,
        })
    }

    pub fn predict(
        &self,
        team: &Team,
        draft: &Draft,
    ) -> HashMap<i64, Vec<ChampionRolePlayerPrediction>> {
        let predictions = self.get_all_predictions(team, draft);

        let drafts = self.create_all_drafts(&predictions);

        let predictions = self.get_predictions_from_drafts(drafts);

        predictions
    }

    pub fn get_all_predictions(
        &self,
        team: &Team,
        draft: &Draft,
    ) -> HashMap<i64, Vec<ChampionRolePlayerPrediction>> {
        let mut predictions = HashMap::new();

        for champion_id in draft.champion_ids.iter() {
            let mut champion_predictions = Vec::new();
            let mut total_probability = 0.0;

            for player in team.players.iter() {
                for role in (1..=5).map(|i| Role::from_i64(i as i64)) {
                    let probability = self.calculate_probability(*champion_id, &role, player);
                    if probability == 0.0 {
                        continue;
                    }

                    total_probability += probability;
                    champion_predictions.push(ChampionRolePlayerPrediction::new(
                        *champion_id,
                        role.clone(),
                        player.clone(),
                        probability,
                    ));
                }
            }

            for prediction in champion_predictions.iter_mut() {
                prediction.probability /= total_probability;
            }

            self.sort_prediction_vec(&mut champion_predictions);

            champion_predictions.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());

            predictions.insert(*champion_id, champion_predictions);
        }

        predictions
    }

    pub fn create_all_drafts(
        &self,
        predictions: &HashMap<i64, Vec<ChampionRolePlayerPrediction>>,
    ) -> Vec<DraftPrediction> {
        let champion_ids = self.sort_prediction_keys_by_prediction_amount(&predictions);

        let base_draft = DraftPrediction::default();

        let mut drafts = self.create_drafts_recursive(base_draft, predictions, &champion_ids, 0);

        self.set_drafts_relative_probability(&mut drafts);
        self.sort_drafts_by_probability(&mut drafts);

        drafts
    }

    fn create_drafts_recursive(
        &self,
        base_draft: DraftPrediction,
        predictions: &HashMap<i64, Vec<ChampionRolePlayerPrediction>>,
        champion_ids: &Vec<i64>,
        index: usize,
    ) -> Vec<DraftPrediction> {
        if let Some(next_champion) = champion_ids.get(index) {
            let picked_roles = base_draft.get_picked_roles();
            let picked_players = base_draft.get_picked_players();

            let mut new_drafts = Vec::new();

            for role_player in predictions.get(next_champion).unwrap() {
                if picked_roles.contains(&&role_player.role)
                    || picked_players.contains(&&role_player.summoner_name)
                {
                    continue;
                }

                let new_draft = base_draft.extend(role_player.clone());

                new_drafts.extend(self.create_drafts_recursive(
                    new_draft,
                    predictions,
                    champion_ids,
                    index + 1,
                ));
            }

            return new_drafts;
        } else {
            // If all champions are used in base draft, return only base draft.
            return vec![base_draft];
        }
    }

    pub fn get_predictions_from_drafts(
        &self,
        drafts: Vec<DraftPrediction>,
    ) -> HashMap<i64, Vec<ChampionRolePlayerPrediction>> {
        let mut predictions = HashMap::new();

        for draft in drafts {
            for prediction in draft.champion_role_players {
                let player_roles = predictions
                    .entry(prediction.champion_id)
                    .or_insert(Vec::new());

                if let Some(existing) =
                    player_roles
                        .iter_mut()
                        .find(|p: &&mut ChampionRolePlayerPrediction| {
                            p.summoner_name == prediction.summoner_name && p.role == prediction.role
                        })
                {
                    existing.probability += draft.probability;
                } else {
                    player_roles.push(ChampionRolePlayerPrediction::new(
                        prediction.champion_id,
                        prediction.role,
                        prediction.summoner_name,
                        draft.probability,
                    ));
                }
            }
        }

        self.set_predictions_relative_probability(&mut predictions);
        self.sort_predictions(&mut predictions);

        predictions
    }

    fn set_predictions_relative_probability(
        &self,
        predictions: &mut HashMap<i64, Vec<ChampionRolePlayerPrediction>>,
    ) {
        for (_, predictions) in predictions {
            let total_probability = predictions.iter().map(|d| d.probability).sum::<f64>();

            for prediction in predictions {
                prediction.probability /= total_probability;
            }
        }
    }

    fn sort_predictions(&self, predictions: &mut HashMap<i64, Vec<ChampionRolePlayerPrediction>>) {
        for (_, predictions) in predictions.iter_mut() {
            self.sort_prediction_vec(predictions);
        }
    }

    fn sort_prediction_vec(&self, predictions: &mut Vec<ChampionRolePlayerPrediction>) {
        let total_probability = predictions.iter().map(|d| d.probability).sum::<f64>();

        for prediction in predictions {
            prediction.probability /= total_probability;
        }
    }

    fn set_drafts_relative_probability(&self, drafts: &mut Vec<DraftPrediction>) {
        let total_probability = drafts.iter().map(|d| d.probability).sum::<f64>();

        for draft in drafts {
            draft.probability /= total_probability;
        }
    }

    fn sort_drafts_by_probability(&self, drafts: &mut Vec<DraftPrediction>) {
        drafts.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    }

    fn sort_prediction_keys_by_prediction_amount(
        &self,
        predictions: &HashMap<i64, Vec<ChampionRolePlayerPrediction>>,
    ) -> Vec<i64> {
        let pairs = predictions
            .iter()
            .map(|(id, predictions)| (id, predictions.iter().len()));

        pairs
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .map(|(champion_id, _)| *champion_id)
            .collect()
    }

    fn calculate_probability(&self, champion_id: i64, role: &Role, summoner_name: &str) -> f64 {
        let ugg_champion_games = self.get_ugg_champion_total_games(champion_id);
        let ugg_champion_role_games = self.get_ugg_champion_role_games(champion_id, role);
        let _ugg_total_games = self.get_ugg_total_games();

        let player_champion_games = self.get_player_champion_games(summoner_name, champion_id);
        let player_champion_role_games =
            self.get_player_champion_role_games(summoner_name, champion_id, role);
        let player_role_games = self.get_player_role_games(summoner_name, role);
        let player_total_games = self.get_player_total_games(summoner_name);

        // Probability of role when champion is chosen.
        let _default_role_probability =
            self.probability(ugg_champion_role_games, ugg_champion_games);

        // Probability of player on champion when player is in game.
        let _player_champion_probability =
            self.probability(player_champion_games, player_total_games);

        // Probability of player in role when player is in game.
        let _player_role_probability = self.probability(player_role_games, player_total_games);

        // Probability of player on champion in role when player is in game.
        let player_champion_role_probability =
            self.probability(player_champion_role_games, player_total_games);

        player_champion_role_probability
    }

    fn probability(&self, part: i64, total: i64) -> f64 {
        part as f64 / total as f64
    }

    fn get_ugg_total_games(&self) -> i64 {
        self.dataset.games
    }

    fn get_ugg_champion_total_games(&self, champion_id: i64) -> i64 {
        if let Some(data) = self.dataset.champion_stats.get(&champion_id) {
            return data.games;
        }

        0
    }

    fn get_ugg_champion_role_games(&self, champion_id: i64, role: &Role) -> i64 {
        if let Some(data) = self.dataset.champion_stats.get(&champion_id) {
            if let Some(role_stats) = data.stats_by_role.get(role) {
                return role_stats.games;
            }
        }

        0
    }

    fn get_player_total_games(&self, summoner_name: &str) -> i64 {
        if let Some(player_stats) = self.player_stats.get(summoner_name) {
            return player_stats.games;
        }

        0
    }

    fn get_player_champion_games(&self, summoner_name: &str, champion_id: i64) -> i64 {
        if let Some(player_stats) = self.player_stats.get(summoner_name) {
            if let Some(champion_stats) = player_stats.champion_stats.get(&champion_id) {
                return champion_stats.games;
            }
        }

        0
    }

    fn get_player_champion_role_games(
        &self,
        summoner_name: &str,
        champion_id: i64,
        role: &Role,
    ) -> i64 {
        if let Some(player_stats) = self.player_stats.get(summoner_name) {
            if let Some(champion_stats) = player_stats.champion_stats.get(&champion_id) {
                if let Some(role_stats) = champion_stats.stats_by_role.get(&role) {
                    return role_stats.games;
                }
            }
        }

        0
    }

    fn get_player_role_games(&self, summoner_name: &str, role: &Role) -> i64 {
        if let Some(player_stats) = self.player_stats.get(summoner_name) {
            if let Some(role_stats) = player_stats.role_stats.get(role) {
                return role_stats.games;
            }
        }

        0
    }
}
