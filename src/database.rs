use colored::*;
use dirs::home_dir;
use serde::Deserialize;
use sqlx::{
    Sqlite,
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
};
use std::fs;
use std::path::Path;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_config: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database: String,
    pub migrates: String,
}


/// Read the config file and return the database config.
/// if not exist, create it.
pub fn read_config() -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
    let home_dir = home_dir().expect("Failed to get home directory");
    let amount_path = home_dir.join(".rustance/database/");
    if !amount_path.exists() {
        fs::create_dir_all(&amount_path).expect("connot create dir");
    }

    let config_path = home_dir.join(".rustance/config_manager.toml");
    if !&config_path.exists() {
        fs::File::create(&config_path).expect("create config error");
        fs::write(
            &config_path,
            format!(
                "[database_config]\ndatabase = \"{}/wallet.db\"\nmigrates = \"{}/migrates\"",
                &amount_path.to_string_lossy(),
                &amount_path.to_string_lossy()
            ),
        )
        .expect("write config error");
    }

    let migrate_path = amount_path.join("migrates/");
    if !&migrate_path.exists() {
        fs::create_dir_all(&migrate_path).expect("connot create dir");
        fs::File::create(migrate_path.join("20250311140451_init.sql"))
            .expect("create migrates error");
        fs::write(
            migrate_path.join("20250311140451_init.sql"),
            "create table if not exists amount_record (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount INTEGER not null default 0,
    in_or_out BOOLEAN not null,
    append_msg TEXT not null,
    created_at DATETIME not null default CURRENT_TIMESTAMP,
    updated_at DATETIME not null default CURRENT_TIMESTAMP
);",
        )
        .expect("write config error");
    }

    let toml_content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&toml_content)?;
    Ok(config.database_config)
}

/// Initialize the database.
pub async fn init_check_database_all() -> PathBuf {
    let config = read_config().expect("unable to read config");
    let wallet_db = &config.database;
    let migrate_path = &config.migrates;

    let _ = create_database(wallet_db).await;
    let _ = check_database(wallet_db, migrate_path).await;

    PathBuf::from(wallet_db)
}

/// check if database exists, create if not.
async fn create_database(database_name: &str) -> Result<(), Box<dyn Error>> {
    if !Sqlite::database_exists(database_name)
        .await
        .unwrap_or(false)
    {
        println!(
            "{} {}",
            "Creating database".green().bold(),
            database_name.blue().bold()
        );
        match Sqlite::create_database(database_name).await {
            Ok(_) => println!(
                "{}{} {}",
                "Create db:".green().bold(),
                database_name.blue().bold(),
                "success!".green().bold()
            ),
            Err(error) => panic!("{}{}", "error: ".red().bold(), error),
        }
    }
    Ok(())
}

/// Read the directory `<migrate_path>/<<timestamp>-<name>.sql>`
/// and execute the sql file to migrate.
async fn check_database(database_name: &str, migrate_path: &str) -> Result<(), Box<dyn Error>> {
    let m = Migrator::new(Path::new(migrate_path)).await?;
    let pool = SqlitePoolOptions::new().connect(database_name).await?;
    m.run(&pool).await.expect("migrate failed.");
    // println!(
    //     "{}{}{}",
    //     "Migrate ".green().bold(),
    //     database_name.blue().bold(),
    //     " successfully.".green().bold()
    // );

    Ok(())
}
