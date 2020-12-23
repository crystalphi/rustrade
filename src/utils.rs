use std::str::FromStr;

use crate::model::{candle::Candle, open_close::OpenClose};
use binance::model::KlineSummary;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use ta::DataItem;

/// Convert binance Kline to TA DataItem
pub fn _kline_to_data_item(summary: &KlineSummary) -> DataItem {
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
    let open_time_fmt = timestamp_to_datetime(&(summary.open_time as u64));
    let close_time_fmt = timestamp_to_datetime(&(summary.close_time as u64));

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
pub fn timestamp_to_datetime(timestamp: &u64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp((timestamp / 1000) as i64, 0);
    DateTime::from_utc(naive, Utc)
}

/// Convert numeric date to String iso formatted
pub fn _timestamp_to_str(timestamp: &u64) -> String {
    let date_time: DateTime<Utc> = timestamp_to_datetime(timestamp);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_timestamp(date_time: &DateTime<Utc>) -> u64 {
    date_time.timestamp_millis() as u64
}

pub fn _datetime_to_str(date_time: &DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_filename(date_time: &DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn time_to_str(date_time: &DateTime<Utc>) -> String {
    date_time.format("%H:%M:%S").to_string()
}

pub fn str_to_datetime(string: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(string, "%Y-%m-%d %H:%M:%S").unwrap()
}

pub fn str_d(string: &str) -> DateTime<Utc> {
    str_to_datetime(string)
}

/// If candles are sorted ok
pub fn _candles_sorted_ok(candles: &[&Candle]) -> bool {
    let sort_ok = candles.iter().map(Some).fold((true, None::<&&Candle>), |previous, current| {
        let result = if let Some(previous_c) = previous.1 {
            if let Some(current_c) = current {
                previous.0 && (current_c.open_time > previous_c.open_time)
            } else {
                previous.0
            }
        } else {
            previous.0
        };
        (result, current)
    });
    sort_ok.0
}

/// Returns inconsistent candles
pub fn inconsistent_candles(candles: &[&Candle], duration: &Duration) -> Vec<Candle> {
    candles
        .iter()
        .map(Some)
        .fold((Vec::new(), None::<&&Candle>), |mut previous, current| {
            if let Some(previous_c) = previous.1 {
                if let Some(current_c) = current {
                    let previous_d = previous_c.open_time;
                    let current_d = current_c.open_time;
                    if current_d - previous_d != *duration {
                        previous.0.push((*current_c).clone());
                    }
                }
            };
            (previous.0, current)
        })
        .0
}

/// Returns min/max close time from candles list
pub fn min_max_close_time_from_candles(candles: &[&Candle]) -> Option<(OpenClose, OpenClose)> {
    if candles.is_empty() {
        return None;
    }
    let mut min_date = OpenClose::Open(str_to_datetime("2000-01-01 00:00:00"));
    let max_date = candles.iter().map(|c| c.open_close()).fold(min_date, |acc, x| acc.max(x));
    min_date = candles.iter().map(|c| c.open_close()).fold(max_date, |acc, x| acc.min(x));
    Some((min_date, max_date))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn timestamp_to_str_test() {
        let dtu = Utc.ymd(1979, 1, 13).and_hms(11, 30, 0);
        let dts = "1979-01-13 11:30:00";
        assert_eq!(_datetime_to_str(&dtu), dts);
        assert_eq!(str_to_datetime(&dts), dtu);
        let dtm = datetime_to_timestamp(&dtu);
        assert_eq!(_timestamp_to_str(&dtm), dts);
    }

    #[test]
    fn candles_sorted_ok_test() {
        let c1 = Candle::new(
            0,
            "2020-01-12 12:00:00",
            "2020-01-12 12:14:59",
            "BTCUSDT",
            15,
            100.0,
            100.0,
            100.0,
            100.0,
            100.0,
        );
        let c2 = Candle::new(
            0,
            "2020-01-12 12:15:00",
            "2020-01-12 12:29:59",
            "BTCUSDT",
            15,
            100.0,
            100.0,
            100.0,
            100.0,
            100.0,
        );

        let d1 = c1.open_time;
        let d2 = c2.open_time;

        let d15m = Duration::minutes(15);
        assert_eq!(d2 - d1, d15m);

        assert_eq!(_candles_sorted_ok(&[&c1, &c2]), true);
        assert_eq!(_candles_sorted_ok(&[&c2, &c1]), false);
        assert_eq!(_candles_sorted_ok(&[&c1, &c1]), false);
        assert_eq!(_candles_sorted_ok(&[&c2, &c2]), false);

        assert_eq!(inconsistent_candles(&[&c1, &c2], &d15m).len(), 0);
        assert_eq!(inconsistent_candles(&[&c2, &c1], &d15m).len(), 1);
        assert_eq!(inconsistent_candles(&[&c1, &c1], &d15m).len(), 1);
        assert_eq!(inconsistent_candles(&[&c2, &c2], &d15m).len(), 1);

        let c3 = Candle::new(
            0,
            "2020-11-16 01:25:00",
            "2020-11-16 01:29:59",
            "BTCUSDT",
            15,
            100.0,
            100.0,
            100.0,
            100.0,
            100.0,
        );

        let c4 = Candle::new(
            0,
            "2020-11-20 11:15:00",
            "2020-11-20 11:29:59",
            "BTCUSDT",
            15,
            100.0,
            100.0,
            100.0,
            100.0,
            100.0,
        );

        assert_eq!(inconsistent_candles(&[&c3, &c4], &d15m).len(), 1);
    }
}
