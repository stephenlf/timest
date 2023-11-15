use anyhow::Result;
use clap::{Parser, ValueEnum, Subcommand, command};
use chrono::NaiveTime;

const DB_PATH: &str = "/home/sfunk/timest/timest.db3";
const NPT_ADDR: &str = "time.nist.gov:123";

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clock in or out
    #[command(arg_required_else_help = true)]
    Clock(ClockArgs),
    /// View timesheet and reports
    View(ViewArgs),
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
enum IO {
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

#[derive(Parser, Debug)]
struct ClockArgs {
    /// Specify whether you are clocking in or out
    io: IO,
    /// Clock time, 24hr. Defaults to current system time.
    #[arg(short, long)]
    time: Option<NaiveTime>,
}

#[derive(Parser, Debug)]
struct ViewArgs {
    
}

fn main() {
    let cli = Cli::parse();

    let cli = match cli.command {
        Commands::Clock(args) => args,
        _ => todo!()
    };

    let operation = cli.io;
    let time = cli.time.unwrap_or(current_time());

    if check_time().is_err() {
            println!("Whoops! Your system clock appears to be too far out of sync. Try fixing it before running this command");
            panic!();
    }

    let conn = sqlite::open(DB_PATH).unwrap();

    prepare_tables(&conn).expect("Expected available .db3 file");
    add_clock(&conn, time, operation).expect("Expected to be able to write to db");
}

fn add_clock(conn: &sqlite::Connection, time: NaiveTime, operation: IO) -> Result<(), anyhow::Error> {
    let command = "
        INSERT INTO times (
            timestamp, io
        ) VALUES (
            :time, :op
        )
    ";
    
    let mut stmt = conn.prepare(command)?;
    stmt.bind::<&[(_, sqlite::Value)]>(&[
            (":time", time.to_string().into()), 
            (":op", operation.to_string().into()),
            ][..])?;
    
    stmt.next()?;
    Ok(())
}

fn current_time() -> chrono::NaiveTime {
    chrono::Local::now().time()
}

fn prepare_tables(conn: &sqlite::Connection) -> Result<(), anyhow::Error> {
    let command = "
        CREATE TABLE IF NOT EXISTS times (
            id INTEGER PRIMARY KEY NOT NULL,
            timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            io TEXT NOT NULL CHECK(io in ('i', 'o'))
        );
    ";

    conn.execute(command)?;
    Ok(())
}

fn get_time() -> u64 {
    let response = ntp::request(NPT_ADDR).unwrap();
    let ntp_time = response.transmit_time.sec;
    ntp_time as u64
}

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn check_time() -> Result<(), &'static str> {
    let sys_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let nist_time = get_time();
    let diff = diff(sys_time + 2208988800_u64, nist_time);
    if diff <= 60*15 {
        Ok(())
    } else {
        Err("System clock is more than one minute out of date")
    }
}