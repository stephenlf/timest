use crate::check_time::*;

const MODIFY_SQL: &str = "
    UPDATE times
    SET timestamp = :timestamp, io = :io
    WHERE id = :id
";

pub fn fix(connection: sqlite::Connection, id: i64, args: crate::ClockArgs) {
    let time = args.time.unwrap_or(
        if check_time().is_err_and(
            |err| prompt_err(&err.to_string()).is_err()
        ) {
            shutdown(connection);
        } else {
            chrono::Local::now().time()
        }
    );
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