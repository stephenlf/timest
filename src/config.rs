pub struct Config {
    pub check_time: bool
}

pub fn make_config_table(conn: &sqlite::Connection) -> Result<(), anyhow::Error> {
    let make_config_table = "
        CREATE TABLE IF NOT EXISTS params (
            parameter TEXT PRIMARY KEY NOT NULL UNIQUE,
            value TEXT
        ) WITHOUT ROWID
    ";

    conn.execute(make_config_table)?;

    Ok(())
}

fn populate_config_table(conn: &sqlite::Connection) -> Result<(), anyhow::Error> {
    // Figure out if there are appropriate keys in config table. If not, insert into.
    todo!()
}