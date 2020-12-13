use chrono::Duration;

use crate::model::candle::Candle;

pub struct CandlesRanges<'a> {
    pub ranges: Vec<Vec<&'a Candle>>,
}

impl<'a> CandlesRanges<'a> {
    pub fn new() -> Self {
        let mut result = Self { ranges: Vec::new() };
        result.new_range();
        result
    }

    pub fn new_range(&mut self) {
        self.ranges.push(Vec::new());
    }

    pub fn push(&mut self, candle: &'a Candle) {
        self.ranges.last_mut().unwrap().push(candle);
    }
}

/// Returns inconsistent candles
pub fn candles_ranges<'a>(candles: &[&'a Candle], duration: &Duration) -> CandlesRanges<'a> {
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
    let ranges = candles_ranges(candles_ref.as_slice(), &Duration::minutes(15 as i64));
    println!("Candles:");
    for (i, range) in ranges.ranges.iter().enumerate() {
        println!(" {} ({}) = {:?}", i, range.len(), range);
    }
}
