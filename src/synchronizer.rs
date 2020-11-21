use chrono::{Duration, Utc};
use ifmt::iprintln;
use rust_decimal_macros::dec;

use crate::{exchange::Exchange, repository::Repository, utils::inconsistent_candles};

pub struct Synchronizer {
    repo: Repository,
    exchange: Exchange,
}

impl Synchronizer {
    pub fn new(repository: Repository, exchange: Exchange) -> Self {
        Synchronizer {
            repo: repository,
            exchange,
        }
    }

    pub fn synchronize(&self, symbol: &str) {
        let mut last_close_time = self.repo.last_close_time(symbol);

        let last_close_time =
            last_close_time.get_or_insert_with(|| Utc::now() - Duration::days(90));

        let d1 = dec!(1);

        let mut candles =
            self.exchange
                .candles(symbol, &15, &Some(*last_close_time + Duration::minutes(15)));

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
    }

    pub fn delete_inconsist(&self) {
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(90);
        let repo = Repository::new().unwrap();
        let candles = repo
            .candles_by_time("BTCUSDT", &start_time, &end_time)
            .unwrap_or_default();

        iprintln!("Found candles: {candles.len()}");

        let candles_ref: Vec<_> = candles.iter().collect();

        println!("Inconsist candles:");
        let inconsist = inconsistent_candles(candles_ref.as_slice(), &Duration::minutes(15));
        for candle in inconsist.iter() {
            iprintln!("{candle}");
            self.repo.delete_candle(&candle.id);
        }
    }
}
