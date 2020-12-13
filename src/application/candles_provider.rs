use crate::{
    candles_range::candles_to_ranges_missing, config::selection::Selection, exchange::Exchange, model::candle::Candle,
    model::candles_result::CandlesResult, provider::candles_buffer::CandlesBuffer, repository::Repository,
};
use anyhow::anyhow;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(candles_buffer: &'a mut CandlesBuffer, repo: &'a Repository, exch: &'a Exchange) -> Self {
        Self {
            exchange: exch,
            repo,
            candles: Vec::new(),
        }
    }

    pub fn candles_selection(&mut self, selection: Selection) -> Result<Vec<Candle>> {
        let start_time = &selection
            .candles_selection
            .start_time
            .unwrap_or_else(|| Utc::now() - Duration::days(180));

        let end_time = &selection.candles_selection.end_time.unwrap_or_else(Utc::now);

        let candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time, &end_time);

        let mut candles = candles.unwrap_or_default();
        let candles_ref = candles.iter().collect::<Vec<_>>();

        let minutes = selection.candles_selection.symbol_minutes.minutes;

        let ranges_missing = candles_to_ranges_missing(start_time, end_time, &minutes, candles_ref.as_slice());
        for range_missing in ranges_missing.iter() {
            let mut candles_exch = self.exchange.candles(
                &selection.candles_selection.symbol_minutes,
                &Some(range_missing.0),
                &Some(range_missing.1),
            );

            self.repo.add_candles(&candles_exch)?;
            candles.append(&mut candles_exch);
        }

        Ok(candles)
    }
}
