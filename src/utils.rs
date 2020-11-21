use std::str::FromStr;

use binance::model::KlineSummary;
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use ta::DataItem;

use crate::model::candle::Candle;

/// Convert binance Kline to TA DataItem
pub fn kline_to_data_item(summary: &KlineSummary) -> DataItem {
    DataItem::builder()
        .open(summary.open)
        .high(summary.high)
        .low(summary.low)
        .close(summary.close)
        .volume(summary.volume)
        .build()
        .unwrap()
}

pub fn fdec(value: f64) -> Decimal {
    Decimal::from_str(&value.to_string()).unwrap()
}

// Convert binance Kline to app Candle
pub fn kline_to_candle(summary: &KlineSummary, symbol: &str, minutes: u32, id: &Decimal) -> Candle {
    let open_time_fmt = fmt_timestamp(summary.open_time);
    let close_time_fmt = fmt_timestamp(summary.close_time);

    Candle {
        id: *id,
        symbol: symbol.into(),
        minutes: minutes.into(),
        open: fdec(summary.open),
        open_time: open_time_fmt,
        high: fdec(summary.high),
        low: fdec(summary.low),
        close: fdec(summary.close),
        volume: fdec(summary.volume),
        close_time: close_time_fmt,
    }
}

/// Convert numeric date to String iso formatted
pub fn fmt_timestamp(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp / 1000, 0);
    let date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}
