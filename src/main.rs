mod api;
mod cli;
mod domain;
mod infra;
mod ui;

use anyhow::Result;
use cli::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run().await
}
