use crate::{ReportArgs, ReportStyle};

use simple_report::simple_report;

mod fancy_report;
use fancy_report::fancy_report;


pub fn report_cmd(conn: sqlite::Connection, args: ReportArgs) {
    let report_style = args.report_style.unwrap_or(ReportStyle::Fancy);
    let date = if args.yesterday {
        chrono::Local::now().date_naive().pred_opt().expect("People should not be clocking in at NaiveDate::MIN")
    } else {
        args.date.unwrap_or(chrono::Local::now().date_naive())
    };
    match report_style {
        ReportStyle::Simple => simple_report(&conn, date).unwrap(),
        ReportStyle::Fancy => fancy_report(&conn, date).unwrap(),
    }
}

const SQL_TODAYS_CLOCK: &str = "
    SELECT * FROM times
    WHERE Date(timestamp) = :date
    ORDER BY timestamp
";

mod simple_report {
    use chrono::{NaiveDate, NaiveDateTime};
    pub fn simple_report(conn: &sqlite::Connection, date: NaiveDate) -> Result<(), anyhow::Error> {
        let date = date.format("%Y-%m-%d").to_string();
        
        println!("Gathering data from day {date}");
        let mut stmt = conn.prepare(super::SQL_TODAYS_CLOCK)?;
        stmt.bind((1, date.as_str()))?;
        println!("====TODAY'S TIMESHEET====");
        println!("->>    {}", &sql_date(conn)?);
        println!(" ________________________");

        for row in stmt.iter() {
            if let Ok(mut result) = row {
                let idx: i64 = result.read(0);
                let timestamp: String = result.take(1).try_into().unwrap();
                let timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
                let io: String = result.take(2).try_into().unwrap();
                println!("|  {idx}  |  {}  |  {io}  |",timestamp.time());
            } else {
                println!("|   |  Bad row 💀");
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

