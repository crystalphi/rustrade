use std::{cmp::Ordering, convert::TryFrom, fmt, ops::Add, ops::Sub};

use anyhow::bail;
use chrono::{DateTime, Duration, Timelike, Utc};

use crate::{
    candles_range::minutes_open_trunc,
    utils::{str_d, str_to_datetime},
};
#[derive(Debug, Eq, Copy, Clone)]
pub enum OpenClose {
    Open(DateTime<Utc>),
    Close(DateTime<Utc>),
    OpenClose(DateTime<Utc>, DateTime<Utc>),
}

impl PartialEq for OpenClose {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OpenClose::Open(self_open), OpenClose::Open(other_open)) => self_open == other_open,
            (OpenClose::Open(_self_open), OpenClose::Close(_other_close)) => false,
            (OpenClose::Open(self_open), OpenClose::OpenClose(other_open, _other_close)) => self_open == other_open,
            (OpenClose::Close(_self_close), OpenClose::Open(_other_open)) => false,
            (OpenClose::Close(self_close), OpenClose::Close(other_close)) => self_close == other_close,
            (OpenClose::Close(self_close), OpenClose::OpenClose(_other_open, other_close)) => self_close == other_close,
            (OpenClose::OpenClose(self_open, _self_close), OpenClose::Open(other_open)) => self_open == other_open,
            (OpenClose::OpenClose(_self_open, self_close), OpenClose::Close(other_close)) => self_close == other_close,
            (OpenClose::OpenClose(self_open, _sc), OpenClose::OpenClose(other_open, _other_close)) => self_open == other_open,
        }
    }
}
impl Ord for OpenClose {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (OpenClose::Open(self_open), OpenClose::Open(other_open)) => self_open.cmp(other_open),
            (OpenClose::Open(_self_open), OpenClose::Close(_other_close)) => Ordering::Equal,
            (OpenClose::Open(self_open), OpenClose::OpenClose(other_open, _other_close)) => self_open.cmp(other_open),
            (OpenClose::Close(_self_close), OpenClose::Open(_other_open)) => Ordering::Equal,
            (OpenClose::Close(self_close), OpenClose::Close(other_close)) => self_close.cmp(other_close),
            (OpenClose::Close(self_close), OpenClose::OpenClose(_other_open, other_close)) => self_close.cmp(other_close),
            (OpenClose::OpenClose(self_open, _self_close), OpenClose::Open(other_open)) => self_open.cmp(other_open),
            (OpenClose::OpenClose(_self_open, self_close), OpenClose::Close(other_close)) => self_close.cmp(other_close),
            (OpenClose::OpenClose(self_open, _sc), OpenClose::OpenClose(other_open, _other_close)) => self_open.cmp(other_open),
        }
    }
}

impl PartialOrd for OpenClose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (OpenClose::Open(self_open), OpenClose::Open(other_open)) => Some(self_open.cmp(other_open)),
            (OpenClose::Open(_self_open), OpenClose::Close(_other_close)) => None,
            (OpenClose::Open(self_open), OpenClose::OpenClose(other_open, _other_close)) => Some(self_open.cmp(other_open)),
            (OpenClose::Close(_self_close), OpenClose::Open(_other_open)) => None,
            (OpenClose::Close(self_close), OpenClose::Close(other_close)) => Some(self_close.cmp(other_close)),
            (OpenClose::Close(self_close), OpenClose::OpenClose(_other_open, other_close)) => Some(self_close.cmp(other_close)),
            (OpenClose::OpenClose(self_open, _self_close), OpenClose::Open(other_open)) => Some(self_open.cmp(other_open)),
            (OpenClose::OpenClose(_self_open, self_close), OpenClose::Close(other_close)) => Some(self_close.cmp(other_close)),
            (OpenClose::OpenClose(self_open, _sc), OpenClose::OpenClose(other_open, _other_close)) => Some(self_open.cmp(other_open)),
        }
    }
}

impl fmt::Display for OpenClose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenClose::Open(open) => write!(f, "{}", open),
            OpenClose::Close(close) => write!(f, "{}", close),
            OpenClose::OpenClose(open, _close) => write!(f, "{}", open),
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

    pub fn from_date(date_time: &DateTime<Utc>, minutes: &u32) -> OpenClose {
        let open = minutes_open_trunc(date_time, minutes);
        let close = open + Duration::minutes(*minutes as i64) - Duration::seconds(1);
        OpenClose::OpenClose(open, close)
    }

    pub fn from_date_close(close: &DateTime<Utc>, minutes: &u32) -> OpenClose {
        let close = *close;
        let open = close + Duration::seconds(1) - Duration::minutes(*minutes as i64);
        OpenClose::OpenClose(open, close)
    }

    pub fn from_str(date_time: &str, minutes: &u32) -> OpenClose {
        let open = minutes_open_trunc(&str_d(date_time), minutes);
        let close = open + Duration::minutes(*minutes as i64) - Duration::seconds(1);
        OpenClose::OpenClose(open, close)
    }
}

impl Add<Duration> for OpenClose {
    type Output = OpenClose;

    fn add(self, other: Duration) -> OpenClose {
        match self {
            OpenClose::Open(open) => OpenClose::Open(open + other),
            OpenClose::Close(close) => OpenClose::Close(close + other),
            OpenClose::OpenClose(open, close) => OpenClose::OpenClose(open + other, close + other),
        }
    }
}

impl Sub<Duration> for OpenClose {
    type Output = OpenClose;

    fn sub(self, other: Duration) -> OpenClose {
        match self {
            OpenClose::Open(open) => OpenClose::Open(open - other),
            OpenClose::Close(close) => OpenClose::Close(close - other),
            OpenClose::OpenClose(open, close) => OpenClose::OpenClose(open - other, close - other),
        }
    }
}

pub fn str_open(date_time: &str) -> OpenClose {
    OpenClose::Open(str_to_datetime(date_time))
}

pub fn str_close(date_time: &str) -> OpenClose {
    OpenClose::Close(str_to_datetime(date_time))
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
