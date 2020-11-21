use std::env;

use binance::{api::Binance, futures::market::FuturesMarket};
use ifmt::iprintln;

use crate::{
    model::candle::Candle,
    utils::{kline_to_candle, kline_to_data_item},
};

pub struct BinanceWrapper {
    api_key: String,
    secret_key: String,
}

impl BinanceWrapper {
    pub fn new() -> BinanceWrapper {
        BinanceWrapper {
            api_key: env::var("API_KEY").unwrap(),
            secret_key: env::var("SECRET_KEY").unwrap(),
        }
    }

    fn futures_market(&self) -> FuturesMarket {
        Binance::new(Some(self.api_key.clone()), Some(self.secret_key.clone()))
    }

    fn candles(&self, symbol: &str, minutes: u32) -> Vec<Candle> {
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
