#[macro_use]
extern crate log;
extern crate time;

mod ugg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let start = time::Instant::now();

    let ranked_data = ugg::profile::get("vigovlugt", "euw1", 16).await?;
    let games = ranked_data.losses.unwrap() + ranked_data.wins.unwrap();
    println!("{:#?} games", games);

    let response = ugg::match_history::get("vigovlugt", games).await;
    for response_data in response {
        println!("{:#?}", response_data)
    }

    println!("{:?} seconds.", start.elapsed().as_seconds_f64());

    Ok(())
}
