#![doc = include_str!("../README.md")]

use clap::Parser;
use std::error::Error;
pub mod cli_customs;
pub mod database;
pub mod process;
use cli_customs::*;
use database::*;
use process::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = init_check_database_all().await;
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListAll(arg) => {
            list_all(arg, database).await?;
        }
        Commands::Income(arg) => {
            insert_in_or_out_come(arg, database, true).await?;
        }
        Commands::Outcome(arg) => {
            insert_in_or_out_come(arg, database, false).await?;
        }
        Commands::PatchRecord(arg) => {
            patch_record(arg, database).await?;
        }
        Commands::DeleteRecord(arg) => {
            delete_record(arg, database).await?;
        }
    }

    Ok(())
}
