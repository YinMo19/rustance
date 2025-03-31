use crate::PatchRecordArgs;
use crate::cli_customs::{AmountArgs, DeleteRecordArgs, ListAllArgs};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use colored::*;
use sqlx::prelude::FromRow;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Highlight, Modify, Panel, Width,
        format::Format,
        object::{Rows, Segment},
        style::{Border, BorderColor, LineText, Style},
        themes::Colorization,
    },
};

/// origin data from database
#[derive(FromRow, Clone, Debug, Tabled)]
pub struct Amount {
    pub id: i32,
    pub amount: u32,
    pub in_or_out: bool,
    pub append_msg: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// precise data for output
/// the different between Amount and AmountPrecise is that
/// AmountPrecise's amount is float (amount / 100.0f32).
#[derive(Clone, Debug, Tabled)]

pub struct AmountPrecise {
    pub id: i32,
    pub amount: f32,
    pub in_or_out: bool,
    pub append_msg: String,
    // pub created_at: String,
    pub updated_at: String,
}

/// List all the Wallet Balances.
/// if the args.time is set, will present the specific month's data.
pub async fn list_all(args: &ListAllArgs, database_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let options = SqliteConnectOptions::new().filename(&database_path);
    let pool = SqlitePool::connect_with(options).await?;

    let query = r#"
        SELECT 
            id, amount, in_or_out, append_msg, created_at, updated_at 
        FROM 
            amount_record
    "#;

    let amount = match args.time {
        Some(ref time) => {
            let current_month = inverse_utc8(
                NaiveDate::parse_from_str(&format!("{}-01", time), "%Y-%m-%d")
                    .expect("time format error."),
            );

            let next_month = current_month.with_month(current_month.month() + 1).unwrap();
            let query = format!("{} WHERE updated_at BETWEEN ? AND ?", query);
            sqlx::query_as(&query)
                .bind(current_month.format("%Y-%m-%d %H:%M:%S").to_string())
                .bind(next_month.format("%Y-%m-%d %H:%M:%S").to_string())
                .fetch_all(&pool)
                .await
                .expect("error when select the amount_record with time range specified")
        }
        None => sqlx::query_as(query)
            .fetch_all(&pool)
            .await
            .expect("error with selecting the amount_record"),
    };

    let grouped_amount = group_amount_by_month(&amount);
    let mut sorted_months: Vec<String> = grouped_amount.keys().cloned().collect();
    sorted_months.sort_unstable();

    for month in sorted_months {
        format_output_month_total(&grouped_amount[&month], month).expect("output error.");
    }

    match args.time {
        Some(_) => (),
        None => println!("{}{}", "Total: ".blue().bold(), {
            let hundred_amount = amount.iter().fold(0, |acc, x| {
                acc + x.amount as i32 * if x.in_or_out { 1 } else { -1 }
            }) as f32
                / 100.0;
            hundred_amount.to_string().purple().italic()
        }),
    };

    Ok(())
}

fn inverse_utc8(current: NaiveDate) -> NaiveDateTime {
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let current_month_midnight = NaiveDateTime::new(current, midnight);

    current_month_midnight - chrono::Duration::hours(8)
}

/// use tabled to format the output.
/// this function only format a single month.
fn format_output_month_total(amount: &[Amount], month: String) -> Result<(), Box<dyn Error>> {
    let mut amount_precise: Vec<AmountPrecise> = amount
        .iter()
        .map(|x| {
            let offset: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
            let updated_at_utc_plus_8 = x.updated_at.with_timezone(&offset);
            let updated_at_str = updated_at_utc_plus_8
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

            AmountPrecise {
                id: x.id,
                amount: x.amount as f32 / 100.0,
                in_or_out: x.in_or_out,
                append_msg: x.append_msg.clone(),
                updated_at: updated_at_str,
            }
        })
        .collect();

    let month_total = amount.iter().fold(0, |acc, x| {
        acc + x.amount as i32 * if x.in_or_out { 1 } else { -1 }
    }) as f32
        / 100.0;
    let (abs_month_total, month_in_or_out) = if month_total > 0.0 {
        (month_total, true)
    } else {
        (-month_total, false)
    };
    amount_precise.push(AmountPrecise {
        id: 0,
        amount: abs_month_total,
        in_or_out: month_in_or_out,
        append_msg: "Month Total".to_string(),
        // created_at: "".to_string(),
        updated_at: "".to_string(),
    });

    let mut table = Table::new(amount_precise);
    table
        .with(Style::blank())
        .with(Colorization::columns([
            Color::FG_BRIGHT_WHITE,
            Color::FG_BRIGHT_BLUE,
            Color::FG_BRIGHT_RED,
            Color::FG_BRIGHT_CYAN,
            Color::FG_MAGENTA,
        ]))
        .with(Panel::header(format!("Accounts Of the Month {}\n", month)))
        .modify(Rows::first(), Color::FG_BRIGHT_YELLOW)
        .modify(Rows::first(), Alignment::center())
        .with(Modify::new(Segment::all()).with(Width::wrap(40)))
        .modify(
            Rows::new(..),
            Format::positioned(|c, pos| {
                if pos.col() == 2 {
                    match c.parse::<bool>() {
                        Ok(false) => {
                            let color = Color::FG_BRIGHT_RED;
                            color.colorize(c)
                        }
                        Ok(true) => {
                            let color = Color::FG_BRIGHT_GREEN;
                            color.colorize(c)
                        }
                        _ => c.to_string(),
                    }
                } else {
                    c.to_string() // 其他列保持不变
                }
            }),
        )
        .with(Highlight::new(Rows::single(2)).color(BorderColor::default().top(Color::FG_YELLOW)))
        .with(Highlight::new(Rows::single(2)).border(Border::new().top('━')))
        .with(Highlight::new(Rows::last()).color(BorderColor::default().top(Color::FG_YELLOW)))
        .with(Highlight::new(Rows::last()).border(Border::new().top('━')))
        .with(LineText::new("Statistics", Rows::last()).color(Color::BG_YELLOW | Color::FG_BLACK))
        .to_string();

    println!("{table}\n");

    Ok(())
}

/// group the amount by month.
/// the input amount may include a lot of month, and
/// output is a hashset, which key is the format: "year-month".
fn group_amount_by_month(amount: &Vec<Amount>) -> HashMap<String, Vec<Amount>> {
    let mut month_map: HashMap<String, Vec<Amount>> = HashMap::new();

    for record in amount {
        let offset: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
        let updated_at_utc_plus_8 = record.updated_at.with_timezone(&offset);
        let month = updated_at_utc_plus_8.format("%Y-%m").to_string();
        month_map.entry(month).or_default().push(record.to_owned());
    }

    month_map
}

/// insert a record
pub async fn insert_in_or_out_come(
    args: &AmountArgs,
    database_path: PathBuf,
    in_or_out: bool,
) -> Result<(), Box<dyn Error>> {
    let options = SqliteConnectOptions::new().filename(&database_path);
    let pool = SqlitePool::connect_with(options).await?;

    let query = r#"
        INSERT INTO amount_record (amount, in_or_out, append_msg, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
    "#;

    sqlx::query(query)
        .bind((args.amount * 100.0) as u32)
        .bind(in_or_out)
        .bind(args.add_msg.clone().unwrap_or("".to_string()))
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(&pool)
        .await
        .expect("error when insert the amount_record");

    println!(
        "{}{}{}: {}",
        "Inserted ".yellow().bold(),
        if in_or_out {
            "income ".green().bold()
        } else {
            "outcome ".red().bold()
        },
        "record with amount".yellow().bold(),
        args.amount.to_string().purple().bold()
    );

    Ok(())
}

/// Delete the specified record.
pub async fn delete_record(
    args: &DeleteRecordArgs,
    database_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let options = SqliteConnectOptions::new().filename(&database_path);
    let pool = SqlitePool::connect_with(options).await?;

    let query_select = r#"
        SELECT 
            id, amount, in_or_out, append_msg, created_at, updated_at
        FROM 
            amount_record
        WHERE 
            id = ?
    "#;

    let record = sqlx::query_as::<_, Amount>(query_select)
        .bind(args.id)
        .fetch_all(&pool)
        .await
        .expect("error when select the amount_record");

    assert!(record.len() == 1, "the id is not exist");
    format_output_month_total(&record, record[0].created_at.to_string()).expect("format error");
    println!(
        "{}{}{}{}",
        "Deleted ".red().bold(),
        "record with id ".yellow().bold(),
        args.id.to_string().purple().bold(),
        ". Input Yes(YES/yes/Y/y) to confirm, other to give up."
            .blue()
            .bold()
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if !["yes", "y"].contains(&input.trim().to_lowercase().as_str()) {
        println!("{}", "Give up".red().bold());
        return Ok(());
    }

    let query = r#"
        DELETE FROM amount_record
        WHERE id = ?
    "#;

    sqlx::query(query)
        .bind(args.id)
        .execute(&pool)
        .await
        .expect("error when delete the amount_record");

    println!("{}", "Delete successfully!".green().bold());

    Ok(())
}

pub async fn patch_record(
    args: &PatchRecordArgs,
    database_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let options = SqliteConnectOptions::new().filename(&database_path);
    let pool = SqlitePool::connect_with(options).await?;

    let query_select = r#"
        SELECT 
            id, amount, in_or_out, append_msg, created_at, updated_at
        FROM 
            amount_record
        WHERE 
            id = ?
    "#;

    let record = sqlx::query_as::<_, Amount>(query_select)
        .bind(args.id)
        .fetch_all(&pool)
        .await
        .expect("error when select the amount_record");

    assert!(record.len() == 1, "the id is not exist");
    println!("{}", "Before:".blue().bold());
    format_output_month_total(&record, record[0].created_at.to_string()).expect("format error");
    println!("{}", "Patched:".blue().bold());
    let records_now = Amount {
        id: args.id,
        amount: args
            .amount
            .map_or(record[0].amount, |amount| (amount * 100.0) as u32),

        in_or_out: args.in_or_out.unwrap_or(record[0].in_or_out),
        append_msg: args.add_msg.clone().unwrap_or(record[0].append_msg.clone()),
        created_at: record[0].created_at,
        updated_at: Utc::now(),
    };

    format_output_month_total(&[records_now.clone()], record[0].created_at.to_string())
        .expect("format error");

    println!(
        "{}{}{}{}",
        "Patch ".blue().bold(),
        "record with id ".yellow().bold(),
        args.id.to_string().purple().bold(),
        ". Input Yes(YES/yes/Y/y) to confirm, other to give up."
            .blue()
            .bold()
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if !["yes", "y"].contains(&input.trim().to_lowercase().as_str()) {
        println!("{}", "Give up".red().bold());
        return Ok(());
    }

    let query = r#"
        UPDATE amount_record
        SET amount = ?, in_or_out = ?, append_msg = ?, updated_at = ?
        WHERE id = ?
    "#;

    let affect_line = sqlx::query(query)
        .bind(records_now.amount)
        .bind(records_now.in_or_out)
        .bind(records_now.append_msg)
        .bind(
            records_now
                .updated_at
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
        .bind(args.id)
        .execute(&pool)
        .await
        .expect("unexpected error when patch the amount_record");

    assert!(affect_line.rows_affected() == 1, "patch error.");
    println!("{}", "Patch successfully!".green().bold());

    Ok(())
}
