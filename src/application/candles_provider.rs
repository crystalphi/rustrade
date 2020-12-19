use crate::{
    candles_range::candles_to_ranges_missing, config::selection::Selection, exchange::Exchange, model::candle::Candle,
    repository::Repository,
};

use anyhow::Result;
use chrono::{Duration, Utc};
use ifmt::iprintln;

use rust_decimal_macros::dec;

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(repo: &'a Repository, exch: &'a Exchange) -> Self {
        Self {
            exchange: exch,
            repo,
            candles: Vec::new(),
        }
    }

    pub fn candles_selection(&mut self, selection: Selection) -> anyhow::Result<Vec<Candle>> {
        iprintln!("Initializing import");

        // Normalize default start/end date time
        let start_time = &selection
            .candles_selection
            .start_time
            .unwrap_or_else(|| Utc::now() - Duration::days(180));

        let end_time = &selection.candles_selection.end_time.unwrap_or_else(Utc::now);

        // Get candles from repository
        let mut candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time, &end_time)
            .unwrap_or_default();

        loop {
            // Get ranges missing
            let ranges_missing = candles_to_ranges_missing(
                start_time,
                end_time,
                &selection.candles_selection.symbol_minutes.minutes,
                candles.iter().collect::<Vec<_>>().as_slice(),
            )?;

            if ranges_missing.is_empty() {
                break;
            }

            for range_missing in ranges_missing.iter() {
                iprintln!("Missing range: {range_missing:?}");

                let mut candles_exch = self.exchange.candles(
                    &selection.candles_selection.symbol_minutes,
                    &Some(range_missing.0),
                    &Some(range_missing.1),
                );

                // Save news candles on repository
                self.repo.add_candles(&mut candles_exch)?;
                // Insert candles on buffer
                candles.append(&mut candles_exch);
            }
        }
        iprintln!("Finished import");

        Ok(candles)
    }
}
