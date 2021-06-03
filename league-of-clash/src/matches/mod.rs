use chrono::{DateTime, NaiveDateTime, Utc};

pub struct Match {
    pub assists: i64,
    pub champion_id: i64,
    pub deaths: i64,
    pub kill_participation: i64,
    pub kills: i64,
    pub match_creation_time: i64,
    pub match_duration: i64,
    pub match_id: i64,
    pub summoner_name: String,
    pub ps_hard_carry: i64,
    pub ps_team_play: i64,
    pub win: bool,
}

impl Match {
    pub fn get_days_ago(self: &Match) -> i64 {
        let now = Utc::now();
        let match_date = NaiveDateTime::from_timestamp(self.match_creation_time / 1000, 0);
        let date = DateTime::from_utc(match_date, Utc);

        now.signed_duration_since::<Utc>(date).num_days()
    }

    pub fn can_use_carry_scores(self: &Match) -> bool {
        (self.ps_hard_carry != 0 && self.ps_team_play != 0)
            && (self.ps_hard_carry < 120 && self.ps_team_play < 120)
    }
}
