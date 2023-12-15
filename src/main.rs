use anyhow::Result;
use platform_dirs::AppDirs;
use std::path::PathBuf;

pub mod args;
use args::*;

mod clock;
use clock::*;

mod report;
use report::*;

mod check_time;

mod fix;
use fix::*;

mod delete;
use delete::del;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let db_path = get_db_path(cli.db_path);
    
    let conn = sqlite::open(db_path).expect("Should be able to open .db3 database");
    prepare_tables(&conn).expect("Expected available .db3 file");

    match cli.command {
        Commands::Clock(args) => clock_cmd(conn, args),
        Commands::Report(args) => report_cmd(conn, args),
        Commands::Fix{id, args} => fix(conn, id, args),
        Commands::Delete { id } => del(conn, id),
    };
    
    Ok(())

}

#[cfg(target_os = "linux")]
const APP_DIR_ERROR: &str = "Could not find $XDG_DATA_HOME or ~/.local/share";

#[cfg(target_os = "macos")]
const APP_DIR_ERROR: &str = "Could not find ~/Library/Application Support";

#[cfg(target_os = "windows")]
const APP_DIR_ERROR: &str = "Could not find %LOCALAPPDATA% (C:\\Users\\%USERNAME%\\AppData\\Local)";



fn get_db_path(user_path: Option<PathBuf>) -> impl AsRef<std::path::Path> + std::fmt::Debug {
    if let Some(path) = user_path {
        return path;
    }
    
    let root_dir = if !cfg!(debug_assertions) {    
        let app_dirs = AppDirs::new(Some("timest"), false).expect(APP_DIR_ERROR);

        let timest_dir = app_dirs.data_dir;
        
        if !timest_dir.is_dir() {
            std::fs::create_dir_all(&timest_dir).expect("Expect to be able to find/modify local app data folder");
        }

        timest_dir
    } else {
        std::env::current_exe().unwrap()
            .parent().unwrap()
            .to_owned()
    };

    root_dir.join("timest.db3")
}

fn prepare_tables(conn: &sqlite::Connection) -> Result<(), anyhow::Error> {
    let make_times_table = "
        CREATE TABLE IF NOT EXISTS times (
            id INTEGER PRIMARY KEY NOT NULL,
            timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            io TEXT NOT NULL CHECK(io in ('i', 'o'))
        );
    ";

    conn.execute(make_times_table)?;

    let make_config_table = "
        CREATE TABLE IF NOT EXISTS params (
            parameter TEXT PRIMARY KEY NOT NULL UNIQUE,
            value TEXT
        ) WITHOUT ROWID
    ";

    conn.execute(make_config_table)?;
    Ok(())
}
