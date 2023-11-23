use crate::{FixArgs, FixCommands};

pub fn fix(connection: sqlite::Connection, args: FixArgs) {
    match args.command {
        FixCommands::Delete => delete(connection, args.id),
        FixCommands::Modify(modify_args) => modify(connection, args.id, modify_args),
    }
}

const DEL_SQL: &str = "
    DELETE FROM times WHERE ID = ?
";

fn delete(connection: sqlite::Connection, id: i64) {
    let mut stmt = connection.prepare(DEL_SQL).unwrap();
    stmt.bind((1, id)).unwrap();
    let _ = stmt.next().unwrap();
}

const MODIFY_SQL: &str = "
    UPDATE times
    SET timestamp = :timestamp, io = :io
    WHERE id = :id
";

fn modify(connection: sqlite::Connection, id: i64, args: crate::ClockArgs) {
    let time = args.time.unwrap_or(chrono::Local::now().time());
    let date = args.date.unwrap_or(chrono::Local::now().date_naive());
    let datetime = date.and_time(time);
    let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let io = args.io.to_string();
    
    let mut stmt = connection.prepare(MODIFY_SQL).unwrap();
    stmt.bind::<&[(_, sqlite::Value)]>(&[
        (":timestamp", datetime_str.into()),
        (":io", io.into()),
        (":id", id.into()),
    ][..]).unwrap();
    let _ = stmt.next().unwrap();
}