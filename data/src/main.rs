use dataset::save_dataset;

use crate::dataset::create_dataset;

mod champion;
mod constants;
mod dataset;
mod riot;
mod ugg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let dataset = create_dataset().await?;
    save_dataset(&dataset, "dataset.json")?;
    save_dataset(&dataset, "../frontend/public/dataset.json")?;

    Ok(())
}
