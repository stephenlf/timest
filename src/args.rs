use clap::{ValueEnum, Subcommand, command};
pub use clap::Parser;
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clock in or out
    #[command(arg_required_else_help = true)]
    Clock(ClockArgs),
    /// View timesheet and reports
    Report(ReportArgs),
}

#[derive(Parser, Debug)]
pub struct ClockArgs {
    /// Specify whether you are clocking in or out
    pub io: IO,
    /// Clock time, 24hr. Defaults to current system time.
    #[arg(short, long)]
    pub time: Option<NaiveTime>,
    /// Clock date. Defaults to today.
    #[arg(short, long)]
    pub date: Option<NaiveDate>,
}

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// See today's raw timesheet
    #[arg(value_enum)]
    pub report_style: ReportStyle,
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