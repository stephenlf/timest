use anyhow::Result;

const DB_PATH: &str = "/home/sfunk/timest/timest.db3";

mod args;
use args::*;

mod clock;
use clock::*;

mod report;
use report::*;

mod check_time;
use check_time::*;

fn main() {
    let cli = Cli::parse();
    
    let conn = sqlite::open(DB_PATH).unwrap();
    prepare_tables(&conn).expect("Expected available .db3 file");

    if check_time().is_err() {
        println!("Whoops! Your system clock appears to be too far out of sync. Try fixing it before running this command");
        panic!();
    }

    match cli.command {
        Commands::Clock(args) => clock_cmd(conn, args),
        Commands::Report(args) => report_cmd(conn, args),
    };

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
