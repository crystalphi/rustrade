use chrono::{DateTime, Utc};
pub struct Serie<'a> {
    pub date_time: &'a DateTime<Utc>,
    pub value: f64,
}

impl<'a> Serie<'a> {
    pub fn new(date_time: &'a DateTime<Utc>, value: f64) -> Self {
        Serie { date_time, value }
    }
}
