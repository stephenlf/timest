use chrono::{NaiveTime, NaiveDateTime};
use sqlite::State;
use crate::IO;

#[derive(Debug, Clone)]
enum Interval {
    Complete {
        start: NaiveTime,
        end: NaiveTime,
    },
    HeadlessOut {
        end: NaiveTime
    },
    TailessIn {
        start: NaiveTime
    }
}

impl Interval {
    pub fn from_records(prev_record: &Option<Record>, current_record: &Option<Record>) -> Option<Self> {
        match (prev_record, current_record) {
            // No complete interval (invalid or otherwise)
            (None, None)                                     | 
            (None, Some(Record(_, IO::I)))                   |
            (Some(Record(_, IO::O)), None)                   |
            (Some(Record(_, IO::O)), Some(Record(_, IO::I))) => None,
            
            // Invalid interval: missing start time
            (None, Some(Record(time, IO::O)))                   |
            (Some(Record(_, IO::O)), Some(Record(time, IO::O))) => Some(
                Self::HeadlessOut { 
                    end: time.clone() 
                }),

            // Invalid interval: missing end time
            (Some(Record(time, IO::I)), None)                   |
            (Some(Record(time, IO::I)), Some(Record(_, IO::I))) => Some(
                Self::TailessIn { 
                    start: time.clone() 
                }),

            // Complete interval
            (Some(Record(time_in, IO::I)), Some(Record(time_out, IO::O))) => Some(
                Self::Complete { 
                    start: time_in.clone(), 
                    end: time_out.clone() 
                }),
        }
    }
}

#[derive(Debug, Clone)]
struct Record(NaiveTime, IO);

impl Record {
    pub fn from_statement(stmt: &sqlite::Statement) -> Result<Self, anyhow::Error> {
        let timestamp: String = stmt.read::<String, _>(1)?;
        let time = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")?
            .time();
        let operation  = IO::try_from(
            stmt.read::<String, _>(2)?
            .as_str()
        )?;
        Ok(Self(time, operation))
    }
}

pub fn fancy_report(conn: &sqlite::Connection) -> Result<(), anyhow::Error> {
    let mut stmt = conn.prepare(super::SQL_TODAYS_CLOCK)?;

    let mut intervals: Vec<Interval> = vec![];
    let mut prev_record: Option<Record> = None;

    while let State::Row = stmt.next()? {
        let current_record = Some(Record::from_statement(&stmt)?);
        if let Some(interval) = Interval::from_records(&prev_record, &current_record) {
            intervals.push(interval)
        }
        prev_record = current_record;
    }

    // Parse last record
    if let Some(interval) = Interval::from_records(&prev_record, &None) {
        intervals.push(interval)
    }

    println!("{intervals:?}");

    Ok(())
}

#[cfg(test)]
mod fancy_tests {
    use super::*;
    use crate::report::report_cmd;

    fn mk_db() -> sqlite::Connection {
        let conn = sqlite::open(":memory:").unwrap();
        crate::prepare_tables(&conn).unwrap();

        let sql = "
            INSERT INTO times (timestamp, io) 
            VALUES
                ('2000-01-01 05:00:00', 'i'),
                ('2000-01-01 06:00:00', 'o'),
                ('2000-01-01 07:00:00', 'i'),
                ('2000-01-01 08:00:00', 'o'),

                ('2001-01-01 10:00:00', 'i'),
                ('2001-01-01 09:00:00', 'o'),
                ('2001-01-01 08:00:00', 'i'),
                ('2001-01-01 07:00:00', 'o'),

                ('2002-01-01 07:00:00', 'o'),
                ('2002-01-01 17:00:00', 'i'),
                
                ('2003-01-01 07:00:00', 'i'),
                ('2003-01-01 08:00:00', 'i'),
                ('2003-01-01 09:00:00', 'o'),
                ('2003-01-01 10:00:00', 'o')
        ";

        conn.execute(sql).unwrap();

        conn
    }

    #[test]
    fn test_writes() {
        let conn = mk_db();
    }
}