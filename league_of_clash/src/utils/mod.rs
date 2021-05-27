use chrono::prelude::*;

pub fn get_current_season() -> i64 {
    let local: DateTime<Local> = Local::now();
    let season = (local.year() - 2013) * 2;

    return season as i64;
}
