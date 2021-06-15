use league_of_clash_shared::role::Role;

use super::champion_role_player_prediction::ChampionRolePlayerPrediction;

#[derive(Clone, Debug)]
pub struct DraftPrediction {
    pub champion_role_players: Vec<ChampionRolePlayerPrediction>,
    pub probability: f64,
}

impl Default for DraftPrediction {
    fn default() -> Self {
        Self {
            champion_role_players: Vec::new(),
            probability: 1.0,
        }
    }
}

impl DraftPrediction {
    pub fn new(champion_role_players: Vec<ChampionRolePlayerPrediction>) -> Self {
        Self {
            probability: Self::probability(&champion_role_players),
            champion_role_players,
        }
    }

    pub fn probability(champion_role_players: &Vec<ChampionRolePlayerPrediction>) -> f64 {
        champion_role_players
            .iter()
            .map(|p| p.probability)
            .product()
    }

    pub fn extend(&self, prediction: ChampionRolePlayerPrediction) -> Self {
        let new_probability = self.probability * prediction.probability;

        let mut new_champion_role_players = self.champion_role_players.clone();
        new_champion_role_players.push(prediction);

        Self {
            probability: new_probability,
            champion_role_players: new_champion_role_players,
        }
    }

    pub fn get_picked_roles(&self) -> Vec<&Role> {
        self.champion_role_players.iter().map(|p| &p.role).collect()
    }

    pub fn get_picked_champions(&self) -> Vec<i64> {
        self.champion_role_players
            .iter()
            .map(|p| p.champion_id)
            .collect()
    }

    pub fn get_picked_players(&self) -> Vec<&String> {
        self.champion_role_players
            .iter()
            .map(|p| &p.summoner_name)
            .collect()
    }
}
