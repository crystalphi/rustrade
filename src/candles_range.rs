use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

use crate::{
    model::candle::Candle,
    utils::{min_max_date_from_candles, str_to_datetime},
};

#[derive(Debug)]
pub struct CandlesRange<'a> {
    candles: Vec<&'a Candle>,
}

impl<'a> CandlesRange<'a> {
    pub fn new() -> Self {
        Self { candles: Vec::new() }
    }

    pub fn push(&mut self, candle: &'a Candle) {
        self.candles.push(candle);
    }

    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }

    pub fn min_max(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        min_max_date_from_candles(self.candles.as_slice())
    }
}

impl<'a> Default for CandlesRange<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug)]
pub struct CandlesRanges<'a> {
    pub ranges: Vec<CandlesRange<'a>>,
}

impl<'a> CandlesRanges<'a> {
    pub fn new() -> Self {
        let mut result = Self { ranges: Vec::new() };
        result.new_range();
        result
    }

    pub fn new_range(&mut self) {
        self.ranges.push(CandlesRange::new());
    }

    pub fn push(&mut self, candle: &'a Candle) {
        self.ranges.last_mut().unwrap().push(candle);
    }
}

impl<'a> Default for CandlesRanges<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn candles_ranges<'a>(candles: &[&'a Candle], minutes: &u32) -> CandlesRanges<'a> {
    let duration = &Duration::minutes(*minutes as i64);
    // Returns inconsistent candles
    candles
        .iter()
        .map(Some)
        .fold((CandlesRanges::new(), None::<&&Candle>), |mut previous, current| {
            if let Some(current_c) = current {
                if let Some(previous_c) = previous.1 {
                    let previous_d = previous_c.open_time;
                    let current_d = current_c.open_time;
                    if current_d - previous_d != *duration {
                        previous.0.new_range();
                    }
                }
                previous.0.push(current_c);
            };
            (previous.0, current)
        })
        .0
}

pub fn invert_ranges(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    ranges: &CandlesRanges,
    minutes: &u32,
) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
    let duration = Duration::minutes(*minutes as i64);
    let mut inverted_ranges = Vec::new();
    let mut prev_start_time = *start_time;
    for range in ranges.ranges.iter() {
        let range_dates = range.min_max();
        let start = prev_start_time;
        let end = range_dates.0 - duration;
        prev_start_time = range_dates.1 + duration;
        inverted_ranges.push((start, end));
    }
    inverted_ranges.push((prev_start_time, *end_time));

    inverted_ranges
}

fn minutes_close_trunc(start_time: &DateTime<Utc>, minutes: &u32) -> DateTime<Utc> {
    let mut start_time = *start_time;

    if start_time.second() == 59 {
        start_time = start_time + Duration::seconds(1);
    };
    let minute = start_time.minute() - (start_time.minute() % minutes);
    start_time = start_time.with_minute(minute).unwrap();
    start_time = start_time - Duration::seconds(1);
    start_time
}

fn candles_to_ranges_missing(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    minutes: &u32,
    candles: &[&Candle],
) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
    let start_time = minutes_close_trunc(start_time, minutes);
    let end_time = minutes_close_trunc(end_time, minutes);

    let candles_ranges = candles_ranges(candles, minutes);
    invert_ranges(&start_time, &end_time, &candles_ranges, minutes)
}

#[cfg(test)]
pub mod testes {
    use std::println;

    use super::*;

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

        let candles = vec![c1, c2, c3, c4];
        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges = candles_ranges(candles_ref.as_slice(), &15);
        println!("Candles:");
        for (i, range) in ranges.ranges.iter().enumerate() {
            println!(" {} ({}) = {:?}", i, range.len(), range);
        }

        let start_time = str_to_datetime("2020-01-01 00:00:00") - Duration::seconds(1);
        let end_time = str_to_datetime("2020-11-30 00:00:00") - Duration::seconds(1);

        let inverted_ranges = invert_ranges(&start_time, &end_time, &ranges, &15);

        println!("Inverted ranges:");
        for inverted_range in inverted_ranges.iter() {
            println!("{} - {}", inverted_range.0, inverted_range.1);
        }
    }

    #[test]
    fn minutes_close_trunc_test() {
        let truncated = minutes_close_trunc(&str_to_datetime("2020-01-01 00:00:00"), &15);

        assert_eq!(truncated, str_to_datetime("2019-12-31 23:59:59"));

        let truncated = minutes_close_trunc(&str_to_datetime("2020-01-01 00:15:00"), &15);
        assert_eq!(truncated, str_to_datetime("2020-01-01 00:14:59"));

        let truncated = minutes_close_trunc(&str_to_datetime("2020-01-01 00:20:00"), &15);
        assert_eq!(truncated, str_to_datetime("2020-01-01 00:14:59"));

        let truncated = minutes_close_trunc(&str_to_datetime("2020-01-01 00:31:00"), &15);
        assert_eq!(truncated, str_to_datetime("2020-01-01 00:29:59"));

        println!("{}", truncated);
    }
}
