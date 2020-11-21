use std::env;

use binance::{api::Binance, futures::market::FuturesMarket};
use chrono::{DateTime, Utc};
use ifmt::iprintln;

use crate::{
    model::candle::Candle,
    utils::{kline_to_candle, kline_to_data_item},
};

use anyhow::Result;

pub struct Exchange {
    api_key: String,
    secret_key: String,
}

impl Exchange {
    pub fn new() -> Result<Exchange> {
        Ok(Exchange {
            api_key: env::var("API_KEY")?,
            secret_key: env::var("SECRETKEY")?,
        })
    }

    pub fn futures_market(&self) -> FuturesMarket {
        Binance::new(Some(self.api_key.clone()), Some(self.secret_key.clone()))
    }

    pub fn candles(
        &self,
        symbol: &str,
        minutes: u32,
        start_time: Option<DateTime<Utc>>,
    ) -> Vec<Candle> {
        let mut result = Vec::new();

        let market = self.futures_market();

        match market.get_klines(symbol.to_string(), "15m", 60, None, None) {
            Ok(answer) => match answer {
                binance::model::KlineSummaries::AllKlineSummaries(summaries) => {
                    for summary in summaries {
                        let dt = kline_to_data_item(&summary);

                        // let ema_val = ema_9.next(&dt);

                        // 17,34,72
                        // let mut macd = Macd::new(3, 6, 4).unwrap();
                        // macd.next(&dt);

                        let candle = kline_to_candle(&summary, &symbol, 15u32, &0u32.into());

                        iprintln!("{candle.open_time}");
                        result.push(candle);
                        //iprintln!("{open_time_fmt}: close {summary.close} vol {summary.volume} : {ema} = {ema_val:2.2}");

                        // repository.insert_candle(candle);

                        // current_id += bd1();
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        result
    }
}
