use crate::{
    model::{
        candle::Candle,
        open_close::{str_open, OpenClose},
    },
    utils::min_max_close_time_from_candles,
};
use anyhow::*;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
use log::error;

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

    pub fn min_max_close(&self) -> anyhow::Result<(OpenClose, OpenClose)> {
        min_max_close_time_from_candles(self.candles.as_slice()).context("CandlesRange.min_max: Candles is empty!")
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

    let mut error = String::from("");

    // Returns inconsistent candles
    let result = candles
        .iter()
        .map(Some)
        .fold((CandlesRanges::new(), None::<&&Candle>), |mut previous, current| {
            if let Some(current_c) = current {
                if let Some(previous_c) = previous.1 {
                    let previous_d = previous_c.open_time;
                    let current_d = current_c.open_time;

                    if previous_d > current_d {
                        error = format!("Previous date {} > current {}", previous_d, current_d);
                    }

                    if current_d == previous_d {
                        error = format!("Found duplicate start x end! Candles list {}:", candles.len());
                    }

                    if current_d - previous_d != *duration {
                        previous.0.new_range();
                    }
                }
                previous.0.push(current_c);
            };

            (previous.0, current)
        })
        .0;

    if !error.is_empty() {
        error!("{}", error);
        //candles.iter().for_each(|c| error!("candles_ranges: {}", c.to_string()));
        bail!(error);
    }

    Ok(result)
}

pub fn invert_ranges_close(start_time: &OpenClose, end_time: &OpenClose, ranges: &CandlesRanges, minutes: &u32) -> anyhow::Result<Vec<(OpenClose, OpenClose)>> {
    fn add_range(ranges: &CandlesRanges, inverted_ranges: &mut Vec<(OpenClose, OpenClose)>, start: OpenClose, end: OpenClose) -> anyhow::Result<()> {
        if start > end {
            let message = format!("Attempt to add range start {} > end {}", start, end);
            error!("{}, added invert_ranges({}):", message, inverted_ranges.len());
            inverted_ranges.iter().for_each(|r| error!("invert_ranges_close: {:?}", r));

            error!("Ranges ({}):", ranges.ranges.len());
            ranges
                .ranges
                .iter()
                .for_each(|r| error!("invert_ranges ranges: {:?}", r.min_max_close().unwrap()));

            bail!(message);
        }

        //if start != end {
        inverted_ranges.push((start, end));
        //}
        Ok(())
    };
    let mut inverted_ranges = Vec::new();
    let duration = Duration::minutes(*minutes as i64);

    let first_min = ranges.ranges.first().unwrap().min_max_close()?.0;
    if start_time < &first_min {
        add_range(&ranges, &mut inverted_ranges, *start_time, first_min - duration)?;
    }

    for i in 1..ranges.ranges.len() {
        let prev = ranges.ranges.get(i - 1).unwrap().min_max_close()?.1 + duration;
        let curr = ranges.ranges.get(i).unwrap().min_max_close()?.0 - duration;
        add_range(&ranges, &mut inverted_ranges, prev, curr)?;
    }

    let end_max = ranges.ranges.last().unwrap().min_max_close()?.1;
    if end_time > &end_max {
        add_range(&ranges, &mut inverted_ranges, end_max + duration, *end_time)?;
    }

    Ok(inverted_ranges)
}

pub fn minutes_open_trunc(start_time: &DateTime<Utc>, minutes: &u32) -> DateTime<Utc> {
    let mut start_time = *start_time;
    let minute = start_time.minute() - (start_time.minute() % minutes);
    start_time = start_time.with_minute(minute).unwrap().with_second(0).unwrap();
    start_time
}

pub fn candles_to_ranges_missing(
    start_time: &OpenClose,
    end_time: &OpenClose,
    minutes: &u32,
    candles: &[&Candle],
) -> anyhow::Result<Vec<(OpenClose, OpenClose)>> {
    if candles.is_empty() {
        return Ok(vec![(*start_time, *end_time)]);
    }
    let limit_date = str_open("2010-01-01 00:00:00");
    if start_time < &limit_date {
        return Err(anyhow!("Start time {:?} is less than allowed!", start_time));
    }
    if end_time < &limit_date {
        return Err(anyhow!("End time {:?} is less than allowed!", end_time));
    }
    // let start_time = minutes_close_trunc(start_time, minutes);
    // let end_time = minutes_close_trunc(end_time, minutes);

    let candles_ranges = match candles_ranges(candles, minutes) {
        Ok(candles) => candles,
        Err(e) => bail!("candles_to_ranges_missing: {} {} {}", start_time, end_time, e),
    };

    match invert_ranges_close(&start_time, &end_time, &candles_ranges, minutes) {
        Ok(result) => Ok(result),
        Err(e) => {
            error!(
                "error returning invert_ranges ({} {}), candles_ranges({}):",
                start_time,
                end_time,
                candles_ranges.ranges.len(),
            );
            candles_ranges.ranges.iter().for_each(|c| error!("{:?}", c.min_max_close().unwrap()));
            Err(anyhow!("candles_to_ranges_missing: {} {} {}", start_time, end_time, e))
        }
    }
}

#[cfg(test)]
pub mod testes {
    use std::println;

    use crate::{
        model::open_close::_str_close,
        utils::{_datetime_to_str, str_d},
    };

    use super::*;

    pub fn close_time_from_open(minutes: &u32, start: &DateTime<Utc>) -> DateTime<Utc> {
        *start + Duration::minutes(*minutes as i64) - Duration::seconds(1)
    }

    fn candle_test(start: &str) -> Candle {
        let end = _datetime_to_str(&close_time_from_open(&15, &str_d(start)));
        Candle::new(0, start, &end, "BTCUSDT", 15, 100.0, 100.0, 100.0, 100.0, 100.0)
    }

    fn candles_test(starts: &[&str]) -> Vec<Candle> {
        starts.iter().map(|s| candle_test(s)).collect()
    }

    #[test]
    fn min_max_test() {
        let candles = candles_test(&["2020-01-12 12:00:00", "2020-01-12 12:15:00", "2020-11-16 01:15:00", "2020-11-20 11:15:00"]);

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges = candles_ranges(candles_ref.as_slice(), &15).unwrap();
        println!("Ranges:");
        for range in ranges.ranges.iter() {
            let date_range = range.min_max_close().unwrap();
            println!("{} - {}", date_range.0, date_range.1);
        }
        assert_eq!(
            ranges.ranges.get(0).unwrap().min_max_close().unwrap(),
            (OpenClose::Close(str_d("2020-01-12 12:14:59")), OpenClose::Close(str_d("2020-01-12 12:29:59")))
        );
        assert_eq!(
            ranges.ranges.get(1).unwrap().min_max_close().unwrap(),
            (OpenClose::Close(str_d("2020-11-16 01:29:59")), OpenClose::Close(str_d("2020-11-16 01:29:59")))
        );
        assert_eq!(
            ranges.ranges.get(2).unwrap().min_max_close().unwrap(),
            (OpenClose::Close(str_d("2020-11-20 11:29:59")), OpenClose::Close(str_d("2020-11-20 11:29:59")))
        );
    }

    #[test]
    fn invert_ranges_test() {
        let candles = candles_test(&["2020-01-12 12:00:00", "2020-01-12 12:15:00", "2020-11-16 01:15:00", "2020-11-20 11:15:00"]);

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges = candles_ranges(candles_ref.as_slice(), &15).unwrap();
        println!("Ranges:");

        let start_time = OpenClose::Close(str_d("2020-01-01 00:00:00") - Duration::seconds(1));
        let end_time = OpenClose::Close(str_d("2020-11-30 00:00:00") - Duration::seconds(1));

        let inverted_ranges = invert_ranges_close(&start_time, &end_time, &ranges, &15).unwrap();

        println!("Inverted ranges {} {}:", start_time, end_time);
        for inverted_range in inverted_ranges.iter() {
            println!("{} - {}", inverted_range.0, inverted_range.1);
        }

        assert_eq!(*inverted_ranges.get(0).unwrap(), (start_time, _str_close("2020-01-12 11:59:59")));
        assert_eq!(
            *inverted_ranges.get(1).unwrap(),
            (_str_close("2020-01-12 12:44:59"), _str_close("2020-11-16 01:14:59"))
        );
        assert_eq!(
            *inverted_ranges.get(2).unwrap(),
            (_str_close("2020-11-16 01:44:59"), _str_close("2020-11-20 11:14:59"))
        );
        assert_eq!(*inverted_ranges.get(3).unwrap(), (_str_close("2020-11-20 11:44:59"), end_time));
    }

    #[test]
    fn invert_ranges_bigger_test() {
        let candles = candles_test(&["2020-01-12 12:00:00", "2020-01-12 12:15:00", "2020-11-16 01:15:00", "2020-11-20 11:15:00"]);

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges = candles_ranges(candles_ref.as_slice(), &15).unwrap();

        let start_time = OpenClose::Close(str_d("2020-01-01 00:00:00") - Duration::seconds(1));
        let end_time = OpenClose::Close(str_d("2020-11-30 00:00:00") - Duration::seconds(1));

        let inverted_ranges = invert_ranges_close(&start_time, &end_time, &ranges, &15).unwrap();

        println!("Inverted ranges {} {}:", start_time, end_time);
        for inverted_range in inverted_ranges.iter() {
            println!("{} - {}", inverted_range.0, inverted_range.1);
        }

        assert_eq!(*inverted_ranges.get(0).unwrap(), (start_time, _str_close("2020-01-12 11:59:59")));
        assert_eq!(
            *inverted_ranges.get(1).unwrap(),
            (_str_close("2020-01-12 12:44:59"), _str_close("2020-11-16 01:14:59"))
        );
        assert_eq!(
            *inverted_ranges.get(2).unwrap(),
            (_str_close("2020-11-16 01:44:59"), _str_close("2020-11-20 11:14:59"))
        );
        assert_eq!(*inverted_ranges.get(3).unwrap(), (_str_close("2020-11-20 11:44:59"), end_time));
    }

    #[test]
    fn minutes_open_trunc_test() {
        let truncated = minutes_open_trunc(&str_d("2020-01-01 00:00:00"), &15);
        assert_eq!(truncated, str_d("2020-01-01 00:00:00"));

        let truncated = minutes_open_trunc(&str_d("2020-01-01 00:17:00"), &15);
        assert_eq!(truncated, str_d("2020-01-01 00:15:00"));

        let truncated = minutes_open_trunc(&str_d("2020-01-01 00:14:59"), &15);
        assert_eq!(truncated, str_d("2020-01-01 00:00:00"));

        let truncated = minutes_open_trunc(&str_d("2020-01-01 00:31:00"), &15);
        assert_eq!(truncated, str_d("2020-01-01 00:30:00"));

        println!("{}", truncated);
    }

    #[test]
    fn candles_to_ranges_missing_test() {
        let start_time = OpenClose::from_str("2020-01-01 00:00:00", &15);
        let end_time = OpenClose::from_str("2020-11-30 00:00:00", &15);

        let candles = candles_test(&["2020-01-12 12:00:00", "2020-01-12 12:15:00", "2020-11-16 01:15:00", "2020-11-20 11:15:00"]);

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges_missing = candles_to_ranges_missing(&start_time, &end_time, &15, candles_ref.as_slice()).unwrap();

        println!("ranges_missing ({}):", ranges_missing.len());
        for range in ranges_missing.iter() {
            println!("{} - {}", range.0, range.1);
        }

        assert_eq!(
            *ranges_missing.get(0).unwrap(),
            (str_open("2020-01-01 00:00:00"), str_open("2020-01-12 11:45:00")),
            "1"
        );
        assert_eq!(
            *ranges_missing.get(1).unwrap(),
            (str_open("2020-01-12 12:30:00"), str_open("2020-11-16 01:00:00"),),
            "2"
        );
        assert_eq!(
            *ranges_missing.get(2).unwrap(),
            (str_open("2020-11-16 01:30:00"), str_open("2020-11-20 11:00:00"),),
            "3"
        );
        assert_eq!(
            *ranges_missing.get(3).unwrap(),
            (str_open("2020-11-20 11:30:00"), str_open("2020-11-30 00:00:00"),),
            "4"
        );
    }

    #[test]
    fn candles_to_ranges_missing_exact_bound_test() {
        let start_time = OpenClose::from_str("2020-01-12 12:00:00", &15);
        let end_time = OpenClose::from_str("2020-11-20 11:15:00", &15);

        let candles = candles_test(&["2020-01-12 12:00:00", "2020-01-12 12:15:00", "2020-11-16 01:15:00", "2020-11-20 11:15:00"]);

        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges_missing = candles_to_ranges_missing(&start_time, &end_time, &15, candles_ref.as_slice()).unwrap();

        println!("ranges_missing ({}):", ranges_missing.len());
        for range in ranges_missing.iter() {
            println!("{} - {}", range.0, range.1);
        }

        assert_eq!(
            *ranges_missing.get(0).unwrap(),
            (str_open("2020-01-12 12:30:00"), str_open("2020-11-16 01:00:00"),),
            "1"
        );
        assert_eq!(
            *ranges_missing.get(1).unwrap(),
            (str_open("2020-11-16 01:30:00"), str_open("2020-11-20 11:00:00"),),
            "2"
        );
    }

    #[test]
    fn missing_candle_test() {
        let candles = candles_test(&["2020-10-11 09:30:00", "2020-10-11 09:45:00", "2020-10-11 10:15:00", "2020-10-11 10:30:00"]);
        let start_time = OpenClose::from_date(&candles.first().unwrap().open_time, &15);
        let end_time = OpenClose::from_date(&candles.last().unwrap().open_time, &15);
        let candles_ref = candles.iter().collect::<Vec<_>>();
        let ranges_missing = candles_to_ranges_missing(&start_time, &end_time, &15, candles_ref.as_slice()).unwrap();

        let missing_candle = OpenClose::from_str("2020-10-11 10:00:00", &15);

        println!("ranges_missing ({}):", ranges_missing.len());
        for range in ranges_missing.iter() {
            println!("{} - {}", range.0, range.1);
        }
        assert_eq!(
            (ranges_missing.first().unwrap().0, ranges_missing.first().unwrap().1),
            (missing_candle, missing_candle)
        );
    }
}
