use crate::utils::min_max_date_from_candles;

use super::candle::Candle;

pub struct CandlesResult {
    candles: Vec<Candle>,
    start_date: Option<String>,
    end_date: Option<String>,
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
}
