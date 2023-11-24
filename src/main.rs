use anyhow::Result;
use microxdg::Xdg;

mod args;
use args::*;

mod clock;
use clock::*;

mod report;
use report::*;

mod check_time;
use check_time::*;

mod fix;
use fix::*;


fn main() {
    let cli = Cli::parse();

    let xdg = Xdg::new().expect("Please set $HOME or $USER shell variable");

    let root_dir = xdg.data()
        .expect("Expected to find XDG_DATA_HOME or $HOME/.local/share")
        .join("timest");
    
    if !root_dir.is_dir() {
        std::fs::create_dir_all(&root_dir).expect("Expect to be able to find/modify local app data folder");
    }

    let db_path = root_dir.join("timest.db3");
    
    let conn = sqlite::open(db_path).expect("Should be able to open .db3 database");
    prepare_tables(&conn).expect("Expected available .db3 file");

    if check_time().is_err() {
        println!("Whoops! Your system clock appears to be too far out of sync. Try fixing it before running this command");
        panic!();
    }

    match cli.command {
        Commands::Clock(args) => clock_cmd(conn, args),
        Commands::Report(args) => report_cmd(conn, args),
        Commands::Fix(args) => fix(conn, args),
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
