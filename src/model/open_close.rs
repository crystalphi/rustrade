use std::convert::TryFrom;

use anyhow::bail;
use chrono::{DateTime, Duration, Timelike, Utc};

use crate::utils::str_d;
#[derive(Debug)]
pub enum OpenClose {
    Open(DateTime<Utc>),
    Close(DateTime<Utc>),
    OpenClose(DateTime<Utc>, DateTime<Utc>),
}

impl PartialEq for OpenClose {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OpenClose::Open(so), OpenClose::Open(oo)) => so == oo,
            (OpenClose::Open(_so), OpenClose::Close(_oc)) => false,
            (OpenClose::Open(so), OpenClose::OpenClose(oo, _oc)) => so == oo,
            (OpenClose::Close(_sc), OpenClose::Open(_oo)) => false,
            (OpenClose::Close(sc), OpenClose::Close(oc)) => sc == oc,
            (OpenClose::Close(sc), OpenClose::OpenClose(_oo, oc)) => sc == oc,
            (OpenClose::OpenClose(so, _sc), OpenClose::Open(oo)) => so == oo,
            (OpenClose::OpenClose(_so, sc), OpenClose::Close(oc)) => sc == oc,
            (OpenClose::OpenClose(so, _sc), OpenClose::OpenClose(oo, _oc)) => so == oo,
        }
    }
}

impl TryFrom<&str> for OpenClose {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        if value.is_empty() {
            bail!("GreaterThanZero only accepts value superior than zero!")
        } else {
            let date = str_d(value);
            Ok(if date.second() == 59 { OpenClose::Close(date) } else { OpenClose::Open(date) })
        }
    }
}

impl OpenClose {
    pub fn to_dates(&self, minutes: &u32) -> (DateTime<Utc>, DateTime<Utc>) {
        match self {
            OpenClose::OpenClose(o, c) => (*o, *c),
            OpenClose::Open(o) => (*o, *o + Duration::minutes(*minutes as i64) - Duration::seconds(1)),
            OpenClose::Close(c) => (*c - Duration::minutes(*minutes as i64) + Duration::seconds(1), *c),
        }
    }

    pub fn open(&self, minutes: &u32) -> DateTime<Utc> {
        self.to_dates(minutes).0
    }

    pub fn close(&self, minutes: &u32) -> DateTime<Utc> {
        self.to_dates(minutes).1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_open_close_test() {
        let open_close = OpenClose::try_from("2020-01-20 00:00:00").unwrap();
        let date = str_d("2020-01-20 00:00:00");
        assert_eq!(open_close, OpenClose::Open(date));

        let open_close = OpenClose::try_from("2020-01-20 00:00:00").unwrap();
        let date = str_d("2020-01-20 00:00:00");
        assert_eq!(open_close, OpenClose::Open(date));

    }
}
