use chrono::{Duration, Utc};
use rust_decimal_macros::dec;

use crate::{exchange::Exchange, repository::Repository};

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
                .candles(symbol, 15, Some(*last_close_time + Duration::minutes(15)));

        let mut last_id = self.repo.last_id();

        candles.iter_mut().for_each(|c| {
            c.id = {
                last_id += d1;
                last_id
            }
        });

        candles.iter().for_each(|c| {
            self.repo.add_candle(c).unwrap();
        });

        last_id += d1;
    }
}
