use chrono::{NaiveDateTime, TimeZone, Utc};

pub struct DateTimeHelper(NaiveDateTime);

impl DateTimeHelper {
    pub fn new(date: NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn to_timestamp(&self) -> i64 {
        Utc.from_local_datetime(&self.0).unwrap().timestamp()
    }

    pub fn to_rfc3339(&self) -> String {
        Utc.from_local_datetime(&self.0).unwrap().to_rfc3339()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

