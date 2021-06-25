#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate log;

pub mod bans;
pub mod champion_role_player_predictor;
pub mod champion_score;
pub mod champion_stats;
pub mod draft;
pub mod matches;
pub mod player_stats;
pub mod role_stats;
pub mod team;
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod ugg;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(test)]
mod tests {
    use league_of_clash_shared::dataset::Dataset;
    use std::{collections::HashMap, fs};

    use crate::{
        champion_role_player_predictor::{
            champion_role_player_prediction::ChampionRolePlayerPrediction,
            ChampionRolePlayerPredictor,
        },
        draft::Draft,
        team::Team,
        utils,
    };

    #[tokio::test]
    async fn main() {
        let contents = fs::read_to_string("../data/dataset.json")
            .expect("Something went wrong reading the file");

        let dataset = Dataset::from_string(&contents).unwrap();

        let team = Team::new(
            vec![
                String::from("vigovlugt"),
                String::from("infernoshot"),
                String::from("pyyr"),
                String::from("jaxverface"),
                String::from("Limgardis"),
            ],
            String::from("euw1"),
        );

        let stats = team
            .get_player_stats(utils::get_current_season())
            .await
            .unwrap();

        let predictor = ChampionRolePlayerPredictor::new(dataset, stats).expect("Predictor error");

        let predictions =
            predictor.get_all_predictions(&team, &Draft::new(vec![235, 3, 106, 777, 64]));
        print_predictions(&predictions);

        let draft_predictions = predictor.create_all_drafts(&predictions);
        for prediction in draft_predictions.iter() {
            println!("{:#?}", prediction);
        }

        let predictions = predictor.get_predictions_from_drafts(draft_predictions);
        print_predictions(&predictions);
    }

    fn print_predictions(predictions: &HashMap<i64, Vec<ChampionRolePlayerPrediction>>) {
        for (champion_id, predictions) in predictions.iter() {
            println!("{}", champion_id);
            for prediction in predictions {
                println!(
                    "{} {:?} {:.2}",
                    prediction.summoner_name,
                    prediction.role,
                    prediction.probability * 100.0
                );
            }

            println!();
        }
    }
}
