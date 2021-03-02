use std::time::Instant;

use anyhow::bail;
use chrono::{Duration, Utc};
use ifmt::iformat;
use log::{error, info};
use rust_decimal_macros::dec;

use crate::{
    candles_utils::inconsistent_candles,
    config::{candles_selection::CandlesSelection, symbol_minutes::SymbolMinutes},
    exchange::Exchange,
    repository::Repository,
};

pub struct Checker<'a> {
    repo: &'a Repository,
    exchange: &'a Exchange,
    symbol_minutes: &'a SymbolMinutes,
}

impl<'a> Checker<'a> {
    pub fn new(symbol_minutes: &'a SymbolMinutes, repository: &'a Repository, exchange: &'a Exchange) -> Self {
        Checker {
            repo: repository,
            exchange,
            symbol_minutes,
        }
    }

    pub fn synchronize(&self) -> anyhow::Result<()> {
        loop {
            self.repo.delete_last_candle(&self.symbol_minutes);

            let mut last_close_time = self.repo.last_close_time(&self.symbol_minutes);

            // If not found last candle then assume last 180 days
            let last_close_time = last_close_time.get_or_insert_with(|| Utc::now() - Duration::days(180));

            info!("{}", iformat!("Last close time: {last_close_time:?}"));

            let d1 = dec!(1);

            let mut candles = self.exchange.candles(
                &self.symbol_minutes,
                &Some(*last_close_time),
                &None, //  + Duration::minutes(*self.minutes as i64)
            )?;

            let mut last_id = self.repo.last_id();

            // Assign id to new candles
            candles.iter_mut().for_each(|c| {
                c.id = {
                    last_id += d1;
                    last_id
                }
            });

            // ### This is a cool example to demonstrate error handling, where error is a type, it's possible capture, filter and transform (adding some semi-context with tuples)
            // Insert candles on repository with `add_candle` and filter by errors
            let candles_errors = candles
                .iter()
                .map(|c| (c, self.repo.insert_candle(c)))
                .filter(|cr| cr.1.is_err())
                .collect::<Vec<_>>();
            if !candles_errors.is_empty() {
                error!("{}", iformat!("Candles add error: {candles_errors.len()}"));
                bail!("Candles add error");
            }

            info!("{}", iformat!("Imported candles: {candles.len()}"));
            if candles.is_empty() {
                break;
            }
        }
        Ok(())
    }

    pub fn check_inconsist(&self, repo: &Repository, selection: &CandlesSelection) {
        let start = Instant::now();
        let start_time = selection.start_time;
        let end_time = selection.end_time;
        info!("{}", iformat!("Check consistent: {self.symbol_minutes:?} {start_time:?} {end_time:?}"));

        let candles = repo.candles_by_time(&self.symbol_minutes, &start_time, &end_time).unwrap_or_default();

        info!("{}", iformat!("Found candles: {candles.len()}"));

        let candles_ref: Vec<_> = candles.iter().collect();

        let inconsist = inconsistent_candles(candles_ref.as_slice(), &Duration::minutes(self.symbol_minutes.minutes as i64));
        info!("{}", iformat!("Inconsist candles: {inconsist.len()}"));
        for candle in inconsist.iter() {
            info!("{}", iformat!("{candle}"));
        }
        info!("{}", iformat!("Elapsed: {start.elapsed():?}"));
    }

    pub fn delete_inconsist(&self) {
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(180);
        let repo = Repository::new().unwrap();

        let candles = repo.candles_by_time(&self.symbol_minutes, &start_time, &end_time).unwrap_or_default();

        info!("{}", iformat!("Found candles: {candles.len()}"));

        let candles_ref: Vec<_> = candles.iter().collect();

        info!("Inconsist candles:");
        let inconsist = inconsistent_candles(candles_ref.as_slice(), &Duration::minutes(self.symbol_minutes.minutes as i64));
        for candle in inconsist.iter() {
            info!("{}", iformat!("{candle}"));
            self.repo.delete_candle(&candle.id);
        }
    }
}
