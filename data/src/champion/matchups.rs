use std::collections::HashMap;

use league_of_clash_shared::{matchup::Matchup, rank::Rank, region::Region, role::Role};
use serde_json::Value;

use crate::ugg;

// https://stats2.u.gg/lol/1.1/matchups/11_12/ranked_solo_5x5/777/1.4.0.json
// counters: c[0].map((function(t) {
//     var e = t[2];
//     return p += e,
//     {
//         champion_id: t[0],
//         win_rate: o()(100 * (1 - t[1] / e), 2),
//         matches: e,
//         xp_adv_15: o()(t[3] / e * -1, 0),
//         gold_adv_15: o()(t[4] / e * -1, 0),
//         duo_gold_adv_15: o()(t[5] / e * -1, 0),
//         cs_adv_15: o()(t[6] / e * -1, 1),
//         duo_cs_adv_15: o()(t[7] / e * -1, 1),
//         jungle_cs_adv_15: o()(t[8] / e * -1, 1),
//         kill_adv_15: o()(t[9] / e * -1, 2),
//         duo_kill_adv_15: o()(t[10] / e * -1, 2),
//         duo_xp_adv_15: o()(t[11] / e * -1, 0),
//         carry_percentage_15: o()(t[12] / e * -1 * 10, 0),
//         duo_carry_percentage_15: o()(t[13] / e * -1 * 10, 0),
//         team_gold_difference_15: o()(t[14] / e * -1, 0)
//     }
// }

pub async fn get_champion_matchups(
    version: &str,
    champion_id: i64,
) -> Result<HashMap<Role, HashMap<i64, Matchup>>, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Getting matchup stats for: {}", champion_id);

    let res =
        ugg::get_ugg_response::<Value>(&version.replace(".", "_"), "matchups", champion_id).await?;

    log::info!("Gotten matchup stats for: {}", champion_id);

    let data = res
        .get(&Region::World)
        .unwrap()
        .get(&Rank::PlatinumPlus)
        .unwrap();

    let matchups_by_role = data
        .into_iter()
        .map(|(role, value)| {
            let matchup_data = &value[0];

            return (
                *role,
                matchup_data
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|data| {
                        let matchup = Matchup {
                            champion_id: data[0].as_i64().unwrap(),
                            wins: data[1].as_i64().unwrap(),
                            games: data[2].as_i64().unwrap(),
                        };

                        return (matchup.champion_id, matchup);
                    })
                    .collect(),
            );
        })
        .collect();

    Ok(matchups_by_role)
}
