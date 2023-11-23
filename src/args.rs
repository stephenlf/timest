use clap::{ValueEnum, Subcommand, command};
pub use clap::Parser;
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub enum Commands {
    /// Clock in or out
    Clock(ClockArgs),
    /// View timesheet and reports
    Report(ReportArgs),
    /// Fix an entry by ID
    Fix(FixArgs),
}

#[derive(Parser, Debug)]
pub struct FixArgs {
    /// Timesheet table entry ID to modify
    pub id: i64,
    #[command(subcommand)]
    pub command: FixCommands,
}

#[derive(Debug, Subcommand)]
pub enum FixCommands {
    /// Update the time, date, or IO of entry
    Modify(ClockArgs),
    /// Delete an entry
    Delete,
}

#[derive(Parser, Debug, Clone)]
pub struct ClockArgs {
    /// Specify whether you are clocking in or out
    pub io: IO,
    /// Clock time, 24hr. Defaults to current system time. Fmt. HH:MM:SS.
    #[arg(short, long)]
    pub time: Option<NaiveTime>,
    /// Clock date. Defaults to today.
    #[arg(short, long)]
    pub date: Option<NaiveDate>,
}

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Timesheet style. Defaults to fancy.
    #[arg(value_enum)]
    pub report_style: Option<ReportStyle>,
    /// Date to view. Defaults to today. Fmt. YYYY-MM-DD.
    #[arg(short, long)]
    pub date: Option<NaiveDate>,
    /// View yesterday's reports. Overrides the --date option.
    #[arg(short, long)]
    pub yesterday: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportStyle {
    Simple,
    Fancy
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
pub enum IO {
    I,
    O
}

impl std::fmt::Display for IO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I => write!(f, "i"),
            Self::O => write!(f, "o"),
        }
    }
}

impl TryFrom::<&str> for IO {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i" => Ok(Self::I),
            "o" => Ok(Self::O),
            _ => Err(anyhow::anyhow!("Unexpected IO char"))
        }
    }
}