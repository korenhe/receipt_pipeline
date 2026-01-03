mod cli;
mod pipeline;
mod db;
mod model;
mod ocr;
mod ollama;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { input_dir, db } => {
            pipeline::run_scan(&input_dir, &db).await?;
        }
        Command::Query { db, sql } => {
            db::run_query(&db, &sql)?;
        }
    }

    Ok(())
}
