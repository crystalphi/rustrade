use std::collections::HashMap;

use crate::{
    candles_range::candles_to_ranges_missing,
    config::{selection::Selection, symbol_minutes::SymbolMinutes},
    exchange::Exchange,
    model::{candle::Candle, open_close::OpenClose},
    repository::Repository,
    technicals::heikin_ashi,
};
use chrono::{Duration, Utc};
use ifmt::iformat;
use log::{info, warn};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    buffer: HashMap<SymbolMinutes, Vec<Candle>>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(repo: &'a Repository, exch: &'a Exchange) -> Self {
        Self {
            exchange: exch,
            repo,
            buffer: HashMap::new(),
        }
    }

    pub fn candles_selection(&mut self, selection: &Selection) -> anyhow::Result<Vec<Candle>> {
        info!("Initializing import");

        fn candles_to_buf(candles: &mut Vec<Candle>, buff: &mut Vec<Candle>) {
            let candles = candles.iter().collect::<Vec<_>>();
            let candles_ref = candles.as_slice();
            let mut candles_repo = heikin_ashi::heikin_ashi(candles_ref);
            buff.append(&mut candles_repo);
            buff.sort();
        }

        // Normalize default start/end date time
        let start_time = &selection.candles_selection.start_time.unwrap_or_else(|| Utc::now() - Duration::days(180));
        let end_time = &selection.candles_selection.end_time.unwrap_or_else(Utc::now);
        let minutes = &selection.candles_selection.symbol_minutes.minutes;
        let symbol_minutes = &selection.candles_selection.symbol_minutes;

        // Get candles from buffer
        let mut candles_buf = self.buffer.entry(symbol_minutes.clone()).or_default();
        let ranges_missing = candles_to_ranges_missing(
            &OpenClose::from_date(start_time, minutes),
            &OpenClose::from_date(end_time, minutes),
            &selection.candles_selection.symbol_minutes.minutes,
            candles_buf.iter().collect::<Vec<_>>().as_slice(),
        )?;

        for range_missing in ranges_missing.iter() {
            let start_time = range_missing.0;
            let end_time = range_missing.1;

            // Get candles from repository
            let mut candles_repo = self
                .repo
                .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time.open(minutes), &end_time.open(minutes))
                .unwrap_or_default();

            candles_to_buf(&mut candles_repo, &mut candles_buf);

            loop {
                // Get ranges missing
                let ranges_missing = candles_to_ranges_missing(
                    &start_time,
                    &end_time,
                    &selection.candles_selection.symbol_minutes.minutes,
                    candles_repo.iter().collect::<Vec<_>>().as_slice(),
                )?;
                info!("Range {} {}. Missing candles: {}", start_time, end_time, ranges_missing.len());
                if ranges_missing.is_empty() {
                    break;
                }

                for range_missing in ranges_missing.iter() {
                    let msg = iformat!("Missing range: {range_missing:?}").to_string();
                    warn!("{}", msg);

                    let mut candles_exch = self.exchange.candles(
                        &selection.candles_selection.symbol_minutes,
                        &Some(range_missing.0.open(minutes)),
                        &Some(range_missing.1.open(minutes)),
                    )?;

                    // Save news candles on repository
                    self.repo.add_candles(&mut candles_exch)?;

                    // Insert candles on buffer
                    candles_to_buf(&mut candles_exch, &mut candles_buf);
                }
            }
        }

        info!("Finished import");

        //let candles_ref = candles.iter().collect::<Vec<_>>();

        Ok(candles_buf.clone())
    }
}
