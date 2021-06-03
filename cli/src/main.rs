use std::convert::TryFrom;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use league_of_clash::{
    bans::bans::Bans,
    champion_stats::{self, champion_stats::ChampionStats},
    team::Team,
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
        .subcommand(
            SubCommand::with_name("player")
                .about("Get info about player")
                .arg(
                    Arg::with_name("region")
                        .short("r")
                        .long("region")
                        .takes_value(true),
                )
                .arg(Arg::with_name("SUMMONER_NAME").required(true).index(1)),
        )
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("team") {
        run_team(matches).await.unwrap();
    } else if let Some(matches) = matches.subcommand_matches("player") {
        run(matches).await.unwrap();
    }
}

pub async fn run(
    arg_matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let summoner_name = arg_matches.value_of("SUMMONER_NAME").unwrap();
    let region = arg_matches.value_of("region").unwrap_or("euw1");

    let season_id = utils::get_current_season();

    let (summoner_name, champion_stats) =
        champion_stats::get_champion_stats_for_player(&summoner_name, &region, season_id).await?;

    print_player(&summoner_name, &champion_stats);

    Ok(())
}

pub async fn run_team(
    arg_matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    let stats = team.get_champion_stats(utils::get_current_season()).await?;

    for (player, champ_stats) in stats.iter() {
        print_player(&player, &champ_stats);
    }

    let bans = league_of_clash::bans::get_bans(&stats);
    print_bans(&bans);

    Ok(())
}

fn print_player(summoner_name: &String, champion_stats: &Vec<ChampionStats>) {
    println!("{}", summoner_name.blue().bold());
    println!("{}", "Champ\tWinrate\tGames\tCarry\tKDA\tScore".bold());
    for stats in champion_stats.iter().take(15) {
        println!(
            "{:.6}\t{:.2}\t{}\t{:.2}\t{:.2}\t{}",
            riven::consts::Champion::try_from(stats.champion_id as i16).unwrap(),
            stats.winrate() * 100f64,
            stats.games,
            stats.carry_score(),
            stats.kda(),
            format!("{:.2}", stats.score * 100.0).bright_green()
        );
    }
    println!();
    println!();
}

fn print_bans(bans: &Vec<Bans>) {
    println!("{}", "Bans".bold().red());
    println!("{}", "Score\tChamp1\tChamp2\tChamp3".bold());
    for ban in bans.iter().take(15) {
        let mut champions = Vec::new();

        for id in ban.champion_ids.iter() {
            champions.push(
                riven::consts::Champion::try_from(*id as i16)
                    .unwrap()
                    .name(),
            )
        }

        println!(
            "{:.2}\t{:.6}\t{:.6}\t{:.6}",
            ban.priority * 100.0,
            champions[0],
            champions[1],
            champions[2]
        );
    }
    println!();
    println!();
}
