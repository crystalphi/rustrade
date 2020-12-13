use chrono::{DateTime, Utc};

use crate::utils::min_max_date_from_candles;

use super::candle::Candle;

pub struct CandlesResult {
    pub candles: Vec<Candle>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl CandlesResult {
    pub fn new(candles: Vec<Candle>) -> Self {
        let (start_date, end_date) = if !candles.is_empty() {
            let candles_def = candles.iter().collect::<Vec<_>>();
            let (min, max) = min_max_date_from_candles(candles_def.as_slice());
            (Some(min), Some(max))
        } else {
            (None, None)
        };

        Self {
            candles,
            start_date,
            end_date,
        }
    }

    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }
}
