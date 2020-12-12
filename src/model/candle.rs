use chrono::{DateTime, Utc};
use ifmt::iwrite;
use rust_decimal::Decimal;
use std::fmt::Display;

use crate::utils::datetime_to_str;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candle {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub id: Decimal,
    pub symbol: String,
    pub minutes: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl Display for Candle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let close_time_fmt = datetime_to_str(&self.close_time);
        iwrite!(
            f,
            "{self.symbol} [{self.minutes} {self.open_time} {close_time_fmt}] {self.close}"
        )
    }
}
