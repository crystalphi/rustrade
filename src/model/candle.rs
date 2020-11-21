use rust_decimal::Decimal;
use std::fmt::Display;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candle {
    pub open_time: String,
    pub close_time: String,
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
        write!(f, "{:?}", self)
    }
}
