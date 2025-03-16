use clap::{Args, Parser, Subcommand};

/// Cli styles
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[derive(Parser)]
#[command(name = "rustance")]
#[command(version = "0.2")]
#[command(about = "Calculate Your Balances.", long_about = None)]
#[command(styles=get_styles())]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all the Wallet Balances.
    ListAll(ListAllArgs),

    /// Add the new income.
    Income(AmountArgs),

    /// Add the new outcome.
    Outcome(AmountArgs),

    /// patch record.
    PatchRecord(PatchRecordArgs),

    /// delete record.
    DeleteRecord(DeleteRecordArgs),
}

#[derive(Args)]
pub struct ListAllArgs {
    // /// List all the Wallet Balances.
    // /// It will print the verbose information if the verbose is true.
    // ///
    // /// The default output is a table of money and month statistics.
    // /// The verbose output will list all the addtional msg.
    // #[arg(short, long, default_value_t = false)]
    // pub verbose: bool,
    /// Time select.
    #[arg(short, long)]
    pub time: Option<String>,
}

#[derive(Args)]
pub struct AmountArgs {
    /// the amount of money. e.g. 100.00.
    /// The amount must be greater than 0.
    /// and the digit after the decimal point must be less than 2.
    pub amount: f32,

    /// add if some additional message is needed.
    pub add_msg: Option<String>,
}

#[derive(Args)]
pub struct DeleteRecordArgs {
    /// the id of the record.
    /// will show the id of the record before delete.
    pub id: i32,
}

#[derive(Args)]
pub struct PatchRecordArgs {
    /// the id of the record.
    pub id: i32,

    /// the amount of money. e.g. 100.00.
    /// The amount must be greater than 0.
    /// and the digit after the decimal point must be less than 2.
    #[arg(long)]
    pub amount: Option<f32>,

    /// the in or out of the record.
    /// true means income, false means outcome.
    #[arg(short, long)]
    pub in_or_out: Option<bool>,

    /// add if some additional message is needed.
    #[arg(short, long)]
    pub add_msg: Option<String>,
}
