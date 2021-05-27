const SEASON_MIN_GAMES: i64 = 10;
const SEASON_GAMES: i64 = 30;
const SEASON_MISSING_GAMES_PENALTY: f64 = 1f64 - (1f64 / SEASON_GAMES as f64);

const MONTH_MIN_GAMES: i64 = 1;
const MONTH_GAMES: i64 = 5;
const MONTH_MISSING_GAMES_PENALTY: f64 = 1f64 - (1f64 / MONTH_GAMES as f64);

/*
+12% winrate (compared to the average for that champion/role) → score * 2
+100% KDA (compared to the average for that champion/role) → score * 1.33
Players must play a minimum of 10 games to be ranked. If a player has played fewer than 50 games with that champion/role → score * 0.75 for each missing game.
You need to have played at least 1 ranked game in the past 30 days to be ranked with that champion.
If you played less than 5 games with the given champion in the less 30 days, a penalty is applied → score * 0.5 for each missing game
Those rules are exponentials (ie +2 tier → score * 16)
*/
pub fn get_score_for_champion_stats(champion_stats: &ChampionStats) -> f64 {
    if champion_stats.games < SEASON_MIN_GAMES
    //|| champion_stats.games_last_month < MONTH_MIN_GAMES
    {
        return 0f64;
    }

    let mut score = champion_stats.winrate() * 100f64;

    let carry_score = champion_stats.carry_score() - 75f64;
    score += carry_score;

    // IMPL AVERAGE KDA

    // 50 games this season
    if champion_stats.games < SEASON_GAMES {
        let missing_games = SEASON_GAMES - champion_stats.games;
        score *= SEASON_MISSING_GAMES_PENALTY.powi(missing_games as i32);
    }

    // 5 games this month
    // if champion_stats.games_last_month < MONTH_GAMES {
    //     let missing_games = MONTH_GAMES - champion_stats.games_last_month;
    //     score *= MONTH_MISSING_GAMES_PENALTY.powi(missing_games as i32);
    // }

    score
}
