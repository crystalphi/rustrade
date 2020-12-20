use crate::{
    model::candle::Candle,
    utils::{min_max_date_from_candles, str_to_datetime},
};
use anyhow::*;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

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

    pub fn min_max(&self) -> anyhow::Result<(DateTime<Utc>, DateTime<Utc>)> {
        min_max_date_from_candles(self.candles.as_slice()).context("CandlesRange.min_max: Candles is empty!")
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

pub fn candles_ranges<'a>(candles: &[&'a Candle], minutes: &u32) -> anyhow::Result<CandlesRanges<'a>> {
    if candles.is_empty() {
        return Err(anyhow!("candles_ranges: Candles is empty!"));
    }
    let duration = &Duration::minutes(*minutes as i64);
    // Returns inconsistent candles
    let result = candles
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
        .0;
    Ok(result)
}

pub fn invert_ranges(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    ranges: &CandlesRanges,
    minutes: &u32,
) -> anyhow::Result<Vec<(DateTime<Utc>, DateTime<Utc>)>> {
    let duration = Duration::minutes(*minutes as i64);
    let mut inverted_ranges = Vec::new();
    let mut prev_start_time = *start_time;
    for range in ranges.ranges.iter() {
        let range_dates = range.min_max()?;
        let start = prev_start_time;
        let end = range_dates.0 - duration;
        prev_start_time = range_dates.1 + duration;
        inverted_ranges.push((start, end));
    }
    inverted_ranges.push((prev_start_time, *end_time));

    Ok(inverted_ranges)
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

pub fn candles_to_ranges_missing(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    minutes: &u32,
    candles: &[&Candle],
) -> anyhow::Result<Vec<(DateTime<Utc>, DateTime<Utc>)>> {
    if candles.is_empty() {
        return Ok(vec![(*start_time, *end_time)]);
    }
    let limit_date = str_to_datetime("2010-01-01 00:00:00");
    if start_time < &limit_date {
        return Err(anyhow!("Start time {:?} is less than allowed!", start_time));
    }
    if end_time < &limit_date {
        return Err(anyhow!("End time {:?} is less than allowed!", end_time));
    }
    let start_time = minutes_close_trunc(start_time, minutes);
    let end_time = minutes_close_trunc(end_time, minutes);

    let candles_ranges = candles_ranges(candles, minutes)?;
    let result = invert_ranges(&start_time, &end_time, &candles_ranges, minutes)?;
    Ok(result)
}

#[cfg(test)]
pub mod testes {
    use std::println;

    use super::*;

    fn candles_test() -> Vec<Candle> {
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

        vec![c1, c2, c3, c4]
    }

    #[test]
    fn invert_ranges_test() {
        let candles = candles_test();

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges = candles_ranges(candles_ref.as_slice(), &15).unwrap();
        println!("Ranges:");
        for range in ranges.ranges.iter() {
            let date_range = range.min_max().unwrap();
            println!("{} - {}", date_range.0, date_range.1);
        }
        assert_eq!(
            ranges.ranges.get(0).unwrap().min_max().unwrap(),
            (
                str_to_datetime("2020-01-12 12:14:59"),
                str_to_datetime("2020-01-12 12:29:59")
            )
        );
        assert_eq!(
            ranges.ranges.get(1).unwrap().min_max().unwrap(),
            (
                str_to_datetime("2020-11-16 01:29:59"),
                str_to_datetime("2020-11-16 01:29:59"),
            )
        );
        assert_eq!(
            ranges.ranges.get(2).unwrap().min_max().unwrap(),
            (
                str_to_datetime("2020-11-20 11:29:59"),
                str_to_datetime("2020-11-20 11:29:59"),
            )
        );

        let start_time = str_to_datetime("2020-01-01 00:00:00") - Duration::seconds(1);
        let end_time = str_to_datetime("2020-11-30 00:00:00") - Duration::seconds(1);

        let inverted_ranges = invert_ranges(&start_time, &end_time, &ranges, &15).unwrap();

        println!("Inverted ranges:");
        for inverted_range in inverted_ranges.iter() {
            println!("{} - {}", inverted_range.0, inverted_range.1);
        }

        assert_eq!(
            *inverted_ranges.get(0).unwrap(),
            (
                str_to_datetime("2019-12-31 23:59:59"),
                str_to_datetime("2020-01-12 11:59:59")
            )
        );
        assert_eq!(
            *inverted_ranges.get(1).unwrap(),
            (
                str_to_datetime("2020-01-12 12:44:59"),
                str_to_datetime("2020-11-16 01:14:59")
            )
        );
        assert_eq!(
            *inverted_ranges.get(2).unwrap(),
            (
                str_to_datetime("2020-11-16 01:44:59"),
                str_to_datetime("2020-11-20 11:14:59")
            )
        );
        assert_eq!(
            *inverted_ranges.get(3).unwrap(),
            (
                str_to_datetime("2020-11-20 11:44:59"),
                str_to_datetime("2020-11-29 23:59:59")
            )
        );
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

    #[test]
    fn candles_to_ranges_missing_test() {
        let start_time = str_to_datetime("2020-01-01 00:00:00");
        let end_time = str_to_datetime("2020-11-30 00:00:00");

        let candles = candles_test();

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges_missing = candles_to_ranges_missing(&start_time, &end_time, &15, candles_ref.as_slice()).unwrap();

        for range in ranges_missing.iter() {
            println!("{} - {}", range.0, range.1);
        }

        assert_eq!(
            *ranges_missing.get(0).unwrap(),
            (
                str_to_datetime("2019-12-31 23:59:59"),
                str_to_datetime("2020-01-12 11:59:59"),
            )
        );
        assert_eq!(
            *ranges_missing.get(1).unwrap(),
            (
                str_to_datetime("2020-01-12 12:44:59"),
                str_to_datetime("2020-11-16 01:14:59"),
            )
        );
        assert_eq!(
            *ranges_missing.get(2).unwrap(),
            (
                str_to_datetime("2020-11-16 01:44:59"),
                str_to_datetime("2020-11-20 11:14:59"),
            )
        );
        assert_eq!(
            *ranges_missing.get(3).unwrap(),
            (
                str_to_datetime("2020-11-20 11:44:59"),
                str_to_datetime("2020-11-29 23:59:59"),
            )
        );
    }
}