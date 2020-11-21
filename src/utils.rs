use std::str::FromStr;

use binance::model::KlineSummary;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
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
    let open_time_fmt = timestamp_to_str(&summary.open_time);
    let close_time_fmt = timestamp_to_str(&summary.close_time);

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
pub fn timestamp_to_str(timestamp: &i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp / 1000, 0);
    let date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_timestamp(date_time: &DateTime<Utc>) -> i64 {
    date_time.timestamp_millis()
}

pub fn datetime_to_str(date_time: &DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn str_to_datetime(string: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(string, "%Y-%m-%d %H:%M:%S").unwrap()
}

#[test]
fn timestamp_to_str_test() {
    let dtu = Utc.ymd(1979, 1, 13).and_hms(11, 30, 0);
    let dts = "1979-01-13 11:30:00";
    assert_eq!(datetime_to_str(&dtu), dts);
    assert_eq!(str_to_datetime(&dts), dtu);
    let dtm = datetime_to_timestamp(&dtu);
    assert_eq!(timestamp_to_str(&dtm), dts);
}
