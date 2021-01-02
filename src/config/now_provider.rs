use chrono::{DateTime, Utc};

pub trait NowProvider {
    fn now(&self) -> DateTime<Utc>;
}

pub struct MockNowProvider {
    now: DateTime<Utc>,
}

impl MockNowProvider {
    pub fn new() -> Self {
        Self { now: Utc::now() }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = now;
    }
}

impl NowProvider for MockNowProvider {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}
