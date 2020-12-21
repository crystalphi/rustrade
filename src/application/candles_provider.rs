use crate::{
    candles_range::candles_to_ranges_missing,
    config::selection::Selection,
    exchange::Exchange,
    model::{candle::Candle, open_close::OpenClose},
    repository::Repository,
};
use chrono::{Duration, Utc};
use ifmt::iformat;
use log::{info, warn};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(repo: &'a Repository, exch: &'a Exchange) -> Self {
        Self { exchange: exch, repo }
    }

    pub fn candles_selection(&mut self, selection: &Selection) -> anyhow::Result<Vec<Candle>> {
        info!("Initializing import");

        // Normalize default start/end date time
        let start_time = &selection.candles_selection.start_time.unwrap_or_else(|| Utc::now() - Duration::days(180));
        let end_time = &selection.candles_selection.end_time.unwrap_or_else(Utc::now);

        let minutes = &selection.candles_selection.symbol_minutes.minutes;

        // Get candles from repository
        let mut candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time, &end_time)
            .unwrap_or_default();

        let start_time = OpenClose::from_date(start_time, minutes);
        let end_time = OpenClose::from_date(end_time, minutes);

        loop {
            // Get ranges missing
            let ranges_missing = candles_to_ranges_missing(
                &start_time,
                &end_time,
                &selection.candles_selection.symbol_minutes.minutes,
                candles.iter().collect::<Vec<_>>().as_slice(),
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
                candles.append(&mut candles_exch);
            }
        }
        info!("Finished import");

        Ok(candles)
    }
}
