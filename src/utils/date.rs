use chrono::{DateTime, Local, NaiveDateTime, TimeDelta, Timelike};
use chrono::offset::TimeZone;
// use regex::Regex;

// const YEAR_REGEX: &str = r"\d{4}";
// const MONTH_REGEX: &str = r"(0\d|1[0-2])";
// const DAY_REGEX: &str = r"([0-2]\d|3[0-1])";
// const HOUR_REGEX: &str = r"([0-1]\d|2[0-3])";
// const MINUTE_REGEX: &str = r"([0-5]\d)";
// const SECOND_REGEX: &str = r"([0-5]\d)";

fn now() -> DateTime<Local> {
    Local::now()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
}

fn later() -> DateTime<Local> {
    now() + TimeDelta::hours(1)
}

fn tomorrow() -> DateTime<Local> {
    now()
        .with_hour(9).unwrap()
        .with_minute(0).unwrap()
        + TimeDelta::days(1)
}

fn parse(string: &str) -> DateTime<Local> {
    println!("Parsing date: `{}`", string);
    let utc = NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
    println!("Parsed date: {:?}", utc);

    Local.from_local_datetime(&utc.unwrap())
        .unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
}

pub fn parse_date(due: &str) -> DateTime<Local> {
    if due == "later" {
        return later();
    }
    if due == "tomorrow" {
        return tomorrow();
    }
    // TODO: Add regex stuff to guess date format and parse
    return parse(due);
    // panic!("Invalid date format");
}
