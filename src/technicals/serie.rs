use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Serie {
    pub date_time: DateTime<Utc>,
    pub value: f64,
}

impl Serie {
    pub fn new(date_time: DateTime<Utc>, value: f64) -> Self {
        Serie { date_time, value }
    }
}
