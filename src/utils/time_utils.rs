use chrono::prelude::*;

const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

//2023-08-06T07:37:03.524688
const ISO_8601DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub fn now() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format(DEFAULT_DATE_FORMAT).to_string()
}

pub fn parse_date(date: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(date, DEFAULT_DATE_FORMAT).unwrap()
}

//iso 8601 to default date format
pub fn iso_8601_to_default_date_format(date: &str) -> String {
    let date = Utc.datetime_from_str(date, ISO_8601DATE_FORMAT).unwrap();
    date.format(DEFAULT_DATE_FORMAT).to_string()
}
