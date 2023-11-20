use crate::{ReportArgs, ReportStyle};

use simple_report::simple_report;

mod fancy_report;
use fancy_report::fancy_report;


pub fn report_cmd(conn: sqlite::Connection, args: ReportArgs) {
    match args.report_style {
        ReportStyle::Simple => simple_report(&conn, None).unwrap(),
        ReportStyle::Fancy => fancy_report(&conn).unwrap(),
    }
}

const SQL_TODAYS_CLOCK: &str = "
    SELECT * FROM times
    WHERE Date(timestamp) = :date
    ORDER BY timestamp
";

mod simple_report {
    use chrono::{NaiveDate, NaiveDateTime};
    pub fn simple_report(conn: &sqlite::Connection, date: Option<NaiveDate>) -> Result<(), anyhow::Error> {
        let date = date.map_or_else(
            || "DATE('now', 'localtime')".to_string(), 
            |d| d.format("%Y-%m-%d").to_string());
        let mut stmt = conn.prepare(super::SQL_TODAYS_CLOCK)?;
        stmt.bind((1, date.as_str()))?;
        println!("====TODAY'S TIMESHEET====");
        println!("->>    {}\n", &sql_date(conn)?);
        println!(" ________________________");

        for (i, row) in stmt.iter().enumerate() {
            if let Ok(mut result) = row {
                let timestamp: String = result.take(1).try_into().unwrap();
                let timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
                let io: String = result.take(2).try_into().unwrap();
                println!("|  {i}  |  {}  |  {io}  |", timestamp.time());
            } else {
                println!("{i}  |  Bad row 💀");
            }
        }
        Ok(())
    }

    fn sql_date(conn: &sqlite::Connection) -> Result<NaiveDate, anyhow::Error> {
        let date: String = conn.prepare("SELECT DATE()")?
            .iter()
            .next()
            .expect("Expect at least 1 row")?
            .take(0)
            .try_into()?;
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
        Ok(date)
    }
}

