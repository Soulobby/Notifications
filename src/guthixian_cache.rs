use chrono::{DateTime, Timelike, Utc};

pub fn guthixian_cache(date: DateTime<Utc>) -> bool {
    date.hour() % 3 == 0
}
