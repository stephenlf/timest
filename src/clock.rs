use crate::{ClockArgs, IO};
use chrono::NaiveDateTime;

pub fn clock_cmd(conn: sqlite::Connection, args: ClockArgs) {
    let operation = args.io;
    let time = args.time.unwrap_or(current_time());
    let date = args.date.unwrap_or(current_date());
    let datetime = date.and_time(time);
    
    add_clock(&conn, datetime, operation).expect("Expected to be able to write to db");
}

fn add_clock(conn: &sqlite::Connection, datetime: NaiveDateTime, operation: IO) -> Result<(), anyhow::Error> {
    let command = "
        INSERT INTO times (
            timestamp, io
        ) VALUES (
            :datetime, :op
        )
    ";
    
    let mut stmt = conn.prepare(command)?;
    stmt.bind::<&[(_, sqlite::Value)]>(&[
            (":datetime", datetime.format("%Y-%m-%d %H:%M:%S").to_string().into()), 
            (":op", operation.to_string().into()),
            ][..])?;
    
    stmt.next()?;
    Ok(())
}

fn current_time() -> chrono::NaiveTime {
    chrono::Local::now().time()
}

fn current_date() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}