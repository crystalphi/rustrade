use rust_decimal::Decimal;
#[derive(Debug)]
pub struct Candle {
    pub id: Decimal,
    pub symbol: String,
    pub minutes: Decimal,
    pub open_time: String,
    pub close_time: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}
