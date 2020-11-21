use chrono::{Duration, Utc};
use ifmt::iprintln;
use rust_decimal_macros::dec;

use crate::{exchange::Exchange, repository::Repository, utils::inconsistent_candles};

pub struct Synchronizer<'a> {
    repo: &'a Repository,
    exchange: &'a Exchange,
    symbol: &'a str,
    minutes: &'a u32,
}

impl<'a> Synchronizer<'a> {
    pub fn new(
        symbol: &'a str,
        minutes: &'a u32,
        repository: &'a Repository,
        exchange: &'a Exchange,
    ) -> Self {
        Synchronizer {
            repo: repository,
            exchange,
            symbol,
            minutes,
        }
    }

    pub fn synchronize(&self) {
        loop {
            let mut last_close_time = self.repo.last_close_time(&self.symbol);

            // If not found last candle then assume last 180 days
            let last_close_time =
                last_close_time.get_or_insert_with(|| Utc::now() - Duration::days(180));

            iprintln!("Last close time: {last_close_time:?}");

            let d1 = dec!(1);

            let mut candles = self.exchange.candles(
                &self.symbol,
                &self.minutes,
                &Some(*last_close_time), //  + Duration::minutes(*self.minutes as i64)
            );

            let mut last_id = self.repo.last_id();

            // Assign id to new candles
            candles.iter_mut().for_each(|c| {
                c.id = {
                    last_id += d1;
                    last_id
                }
            });

            // Insert candles on repository
            candles.iter().for_each(|c| {
                self.repo.add_candle(c).unwrap();
            });

            iprintln!("Imported candles: {candles.len()}");
            if candles.is_empty() {
                break;
            }
        }
    }

    pub fn check_inconsist(&self) {
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(180);
        let repo = Repository::new().unwrap();
        let candles = repo
            .candles_by_time(&self.symbol, &15, &start_time, &end_time)
            .unwrap_or_default();

        iprintln!("Found candles: {candles.len()}");

        let candles_ref: Vec<_> = candles.iter().collect();

        let inconsist = inconsistent_candles(
            candles_ref.as_slice(),
            &Duration::minutes(*self.minutes as i64),
        );
        iprintln!("Inconsist candles: {inconsist.len()}");
        for candle in inconsist.iter() {
            iprintln!("{candle}");
        }
    }

    pub fn delete_inconsist(&self) {
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(180);
        let repo = Repository::new().unwrap();
        let candles = repo
            .candles_by_time(&self.symbol, &15, &start_time, &end_time)
            .unwrap_or_default();

        iprintln!("Found candles: {candles.len()}");

        let candles_ref: Vec<_> = candles.iter().collect();

        println!("Inconsist candles:");
        let inconsist = inconsistent_candles(
            candles_ref.as_slice(),
            &Duration::minutes(*self.minutes as i64),
        );
        for candle in inconsist.iter() {
            iprintln!("{candle}");
            self.repo.delete_candle(&candle.id);
        }
    }
}
