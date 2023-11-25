# Timest - Dead Simple Punch Card CLI

Timest is a dead simple CLI time card. Clock in. Clock out. See how much you worked.

## Installation
1. With `cargo`
```bash
$ cargo install timest
```
2. Build from source - _must have [cargo/rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed_
```bash
# Linux
$ git clone https://github.com/stephenlf/timest.git
$ cd timest
$ cargo build --release
$ cp target/release/timest ~/.cargo/bin/timest
```

## Usage

### Clock
Clock in and out easily.
```bash
# Clock in/out
$ timest clock i
$ timest clock o
# Clock in/out at a different time or date
$ timest clock i --time 8:00
$ timest clock i --time 8:00 --date 2023-05-31
$ timest clock o --t 8:00 --d 2023-05-31
```

### View Reports
View today's report.
```bash
$ timest clock i -t 11:23:38
$ timest clock o -t 12:09:59
$ timest report
# Output below
```
![Screenshot of the terminal. A timeline from midnight to midnight stretches across the top, with blue plus signs indicating time worked. A box labelled "SUMMARY" has two columns: "INTERVAL" and "DURATION". The "INTERVAL" column has one item: "11:23:38 - 12:09:59". Its corresponding "DURATION" cell reads "00:46:21". A blue notice at the bottom reads "TOTAL TIME WORKED: 0:46:21"](./assets/report_fancy.png)

Get warned of incomplete work intervals and get prompts to fix them.
```bash
$ timest clock i -t  8:00
$ timest clock i -t  13:20:23
$ timest clock i -o  14:00
$ timest report
# Output below
```
![Screenshot of the terminal. A timeline from midnight to midnight stretches across the top, with blue plus signs indicating time worked and red plus signs indicating incomplete intervals. A box labelled "SUMMARY" has two columns: "INTERVAL" and "DURATION". The "INTERVAL" column has two items: one row showing a time interval with a start time but no stop time, and one row showing a complete interval. Only the complete interval has a corresponding "DURATION" cell. At the bottom there is a warning that says "ERROR there are some incomplete intervals". Beneath that, there are instructions on how to fix incomplete intervals.](./assets/report_fancy_error.png)

Run a simple report to see raw timestamps.
```shell
# See today's reports
$ timest report simple
# Gathering data from day 2023-11-25
# ====TODAY'S TIMESHEET====
# ->>    2023-05-31
#  ________________________
# |  7  |  11:23:38  |  i  |
# |  8  |  12:09:59  |  o  |
```

Use the `--date`/`-d` flag to specify a day to view, or use the `--yesterday`/`-y` flag to see yesterday's report.
```shell
$ timest report -y
# ...
```

### Fix Entries
Use `timest report simple` to get entry ids, then run `timest fix {id}` to modify the bad entry. `timest fix {id}` uses the same arguments as `timest clock`.
```shell
$ timest clock i -t 8:00
$ timest report simple
# Gathering data from day 2023-11-25
# ====TODAY'S TIMESHEET====
# ->>    2023-05-31
#  ________________________
# |  9  |  08:00:00  |  i  |
$ timest fix 9 o -t 11:24:38
$ timest report simple
# Gathering data from day 2023-11-25
# ====TODAY'S TIMESHEET====
# ->>    2023-05-31
#  ________________________
# |  9  |  11:24:38  |  o  |
```

### Delete Entries
Use `timest report simple` to get entry ids, then run `timest delete {id}` to remove the bad entry.
```shell
$ timest clock i -t 8:00
$ timest clock i -t 8:01
$ timest report simple
# Gathering data from day 2023-11-25
# ====TODAY'S TIMESHEET====
# ->>    2023-05-31
#  ________________________
# |  10 |  11:24:38  |  o  |
# |  11 |  11:24:38  |  o  |
$ timest delete 11
$ timest report simple
# Gathering data from day 2023-11-25
# ====TODAY'S TIMESHEET====
# ->>    2023-05-31
#  ________________________
# |  10 |  11:24:38  |  o  |
```