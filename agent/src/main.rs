mod app;
mod handles;
mod utils;

use crate::app::config::Config;
use crate::utils::logging::init_tracing;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("  Guardian Agent\n");

    let config = Config::load()?;
    let _ = init_tracing(config.clone())?;

    print!("  Local: http://127.0.0.1:{}", config.port.clone());

    let mut app = app::runner::App::new(config)?;
    app.run().await?;

    Ok(())
}
