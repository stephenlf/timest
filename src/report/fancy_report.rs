use chrono::{NaiveTime, NaiveDate, NaiveDateTime};
use sqlite::State;
use crate::IO;
use recolored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
        start: NaiveTime,
        end: NaiveTime,
        status: IntervalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntervalStatus {
    Complete,
    MissingStart,
    MissingEnd
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
            (None, Some(Record(time, IO::O))) => Some(
                Self { 
                    start: NaiveTime::MIN, 
                    end: *time,
                    status: IntervalStatus::MissingStart
                }),
            (Some(Record(time_a, IO::O)), Some(Record(time_b, IO::O))) => Some(
                Self { 
                    start: *time_a,
                    end: *time_b,
                    status: IntervalStatus::MissingStart
                }),

            // Invalid interval: missing end time
            (Some(Record(time, IO::I)), None) => Some(
                Self { 
                    start: *time,
                    end: NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
                    status: IntervalStatus::MissingEnd
                }),
            (Some(Record(time_a, IO::I)), Some(Record(time_b, IO::I))) => Some(
                Self { 
                    start: *time_a,
                    end: *time_b,
                    status: IntervalStatus::MissingEnd
                }),

            // Complete interval
            (Some(Record(time_in, IO::I)), Some(Record(time_out, IO::O))) => Some(
                Self { 
                    start: *time_in, 
                    end: *time_out,
                    status: IntervalStatus::Complete
                }),
        }
    }

    pub fn contains(&self, time: &NaiveTime) -> bool {
        time > &self.start && time < &self.end
    }

    pub fn duration(&self) -> i64 {
        self.end.signed_duration_since(self.start).num_seconds()
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

pub fn fancy_report(conn: &sqlite::Connection, date: NaiveDate) -> Result<(), anyhow::Error> {
    let mut stmt = conn.prepare(super::SQL_TODAYS_CLOCK)?;

    let date = date.format("%Y-%m-%d").to_string();
    stmt.bind((1, date.as_str()))?;

    let intervals = get_intervals(&mut stmt)?;
    generate_report(&intervals);

    Ok(())
}

fn get_intervals(stmt: &mut sqlite::Statement) -> Result<Vec<Interval>, anyhow::Error> {
    let mut intervals: Vec<Interval> = vec![];
    let mut prev_record: Option<Record> = None;

    while let State::Row = stmt.next()? {
        let current_record = Some(Record::from_statement(stmt)?);
        if let Some(interval) = Interval::from_records(&prev_record, &current_record) {
            intervals.push(interval)
        }
        prev_record = current_record;
    }

    // Parse last record
    if let Some(interval) = Interval::from_records(&prev_record, &None) {
        intervals.push(interval)
    }

    Ok(intervals)
}

fn generate_report(intervals: &[Interval]) {
    print_header();
    print_bar(intervals);
    print_summary(intervals);
    print_total(intervals);
}

fn print_total(intervals: &[Interval]) {
    if let Some(duration) = seconds_worked(intervals) {
        let notice = "TOTAL TIME WORKED: ".blue();
        let time_worked = NaiveTime::from_num_seconds_from_midnight_opt(duration as u32, 0)
            .unwrap()
            .to_string();
        let pretty_time = if let Some(stripped) = time_worked.strip_prefix('0') {
            stripped
        } else {
            time_worked.as_str()
        }.blue().bold();
        println!("{}{}", notice, pretty_time);
    } else {
        print!("{}", "ERROR".red().bold());
        println!(" {}", "there are some incomplete intervals".red());
        println!("\tPlease run `timest report simple` to find the missing records ");
        println!("\tand fix them with `timest clock` and `timest fix`");
    }
}

fn seconds_worked(intervals: &[Interval]) -> Option<i64> {
    let mut total = 0_i64;
    for interval in intervals.iter() {
        if interval.status == IntervalStatus::Complete {
            total += interval.duration();
        } else {
            return None;
        }
    }
    Some(total)
}

fn print_summary(intervals: &[Interval]) {
    println!("            SUMMARY");
    println!("┌───────────────────────┬────────────┐");
    println!("│        INTERVAL       │  DURATION  │");
    println!("├───────────────────────┼────────────┤");
    for interval in intervals.iter() {
        match interval.status {
            IntervalStatus::Complete => println!(
                "│  {} - {}  │  {}  │", 
                interval.start, 
                interval.end, 
                NaiveTime::from_num_seconds_from_midnight_opt(
                    interval.duration() as u32, 0
                ).unwrap()),
            IntervalStatus::MissingEnd => println!(
                "│  {} - ??:??:??  │            │ ",
                interval.start
            ),
            IntervalStatus::MissingStart => println!(
                "│  ??:??:?? - {}  │            │", 
                interval.end
            ),
        }
    }
    println!("└───────────────────────┴────────────┘");
}

fn print_header() {
    print!("mdnt");
    print!("                          6");
    print!("         8");
    print!("         10");
    print!("        noon");
    print!("      2");
    print!("         4");
    println!("         6");
}

fn print_bar(intervals: &[Interval]) {
    // Each dash is a 15 minute interval
    'outer: for i in 0..(24 * 4) {
        if i % 4 == 0 {
            print!("|");
        }
        for interval in intervals.iter() {
            let num_seconds = i * 15 * 60 + 15;
            let time = NaiveTime::from_num_seconds_from_midnight_opt(num_seconds, 0)
                .unwrap();
            if interval.contains(&time) {
                match interval.status {
                    IntervalStatus::Complete => print!("{}", "+".blue()),
                    _ => print!("{}", "+".red().bold()),
                }
                continue 'outer;
            } 
        }
        print!("-");
    }
    println!();
}

