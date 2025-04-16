use chrono::{DateTime, Local, TimeDelta, Timelike};

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

pub fn parse_date(due: &str) -> DateTime<Local> {
    match due {
        "later" => later(),
        "tomorrow" => tomorrow(),
        _ => panic!("Invalid date format"),
    }
}
