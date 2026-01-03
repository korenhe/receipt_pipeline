use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "receipt-pipeline")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Scan {
        #[arg(long)]
        input_dir: String,

        #[arg(long, default_value = "receipts.duckdb")]
        db: String,
    },

    Query {
        #[arg(long)]
        db: String,

        #[arg(long)]
        sql: String,
    },
}
