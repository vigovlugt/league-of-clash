use std::{convert::TryFrom, env};

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use league_of_clash::{
    champion_stats::{self, champion_stats::ChampionStats},
    team::Team,
    utils,
};

// cargo r team vigovlugt infernoshot pyyr jaxverface sneakym0nk3y

#[tokio::main]
async fn main() {
    let app = App::new("League of Clash - CLI")
        .version("3.0")
        .subcommand(
            SubCommand::with_name("team")
                .about("Get info about clash team")
                .arg(
                    Arg::with_name("region")
                        .short("r")
                        .long("region")
                        .takes_value(true),
                )
                .arg(Arg::with_name("SUMMONER_1").required(true).index(1))
                .arg(Arg::with_name("SUMMONER_2").required(true).index(2))
                .arg(Arg::with_name("SUMMONER_3").required(true).index(3))
                .arg(Arg::with_name("SUMMONER_4").required(true).index(4))
                .arg(Arg::with_name("SUMMONER_5").required(true).index(5)),
        )
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("team") {
        run_team(matches).await.unwrap();
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let summoner_name = &args[2];
    let region = &args[1];
    // let _days = &args[3].parse::<i64>()?;
    // let _pow = &args[4].parse::<f64>()?;

    let season_id = utils::get_current_season();

    let (summoner_name, champion_stats) =
        champion_stats::get_champion_stats_for_player(&summoner_name, &region, season_id).await?;

    print_for_player(summoner_name, champion_stats);

    Ok(())
}

fn print_for_player(summoner_name: String, champion_stats: Vec<ChampionStats>) {
    println!("{}", summoner_name);
    println!("Champ\tWinrate\tGames\tCarry\tKDA\tRealWr");
    for stats in champion_stats.iter().take(15) {
        println!(
            "{:.6}\t{:.2}\t{}\t{:.2}\t{:.2}\t{:.2}",
            riven::consts::Champion::try_from(stats.champion_id as i16).unwrap(),
            stats.winrate() * 100f64,
            stats.games,
            stats.carry_score(),
            stats.kda(),
            stats.total_winrate() * 100f64
        );
    }
    println!();
    println!();
}

pub async fn run_team(arg_matches: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let region = arg_matches.value_of("region").unwrap_or("euw1");

    let summoner_name1 = arg_matches.value_of("SUMMONER_1").unwrap();
    let summoner_name2 = arg_matches.value_of("SUMMONER_2").unwrap();
    let summoner_name3 = arg_matches.value_of("SUMMONER_3").unwrap();
    let summoner_name4 = arg_matches.value_of("SUMMONER_4").unwrap();
    let summoner_name5 = arg_matches.value_of("SUMMONER_5").unwrap();

    let players = vec![
        summoner_name1.to_owned(),
        summoner_name2.to_owned(),
        summoner_name3.to_owned(),
        summoner_name4.to_owned(),
        summoner_name5.to_owned(),
    ];

    let team = Team::new(players, region.to_owned());

    let stats = team.get_champion_stats().await?;

    for (player, champ_stats) in stats {
        print_for_player(player, champ_stats);
    }

    Ok(())
}
