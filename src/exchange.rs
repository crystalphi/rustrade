use std::env;

use binance::{api::Binance, futures::market::FuturesMarket};
use chrono::{DateTime, Utc};
use ifmt::{iformat, iprintln};

use crate::{
    config::symbol_minutes::SymbolMinutes,
    model::candle::Candle,
    utils::{datetime_to_timestamp, kline_to_candle},
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
            secret_key: env::var("SECRET_KEY")?,
        })
    }

    pub fn futures_market(&self) -> FuturesMarket {
        Binance::new(Some(self.api_key.clone()), Some(self.secret_key.clone()))
    }

    pub fn candles(
        &self,
        symbol_minutes: &SymbolMinutes,
        start_time: &Option<DateTime<Utc>>,
        end_time: &Option<DateTime<Utc>>,
    ) -> Vec<Candle> {
        let mut result = Vec::new();

        let start_time = start_time.map(|d| datetime_to_timestamp(&d));
        let end_time = end_time.map(|d| datetime_to_timestamp(&d));

        let market = self.futures_market();

        match market.get_klines(
            symbol_minutes.symbol.to_string(),
            iformat! {"{symbol_minutes.minutes}m"},
            1000,
            start_time,
            end_time,
        ) {
            Ok(answer) => match answer {
                binance::model::KlineSummaries::AllKlineSummaries(summaries) => {
                    for summary in summaries {
                        let candle =
                            kline_to_candle(&summary, &symbol_minutes.symbol, symbol_minutes.minutes, &0u32.into());
                        iprintln!("{candle.open_time}");
                        result.push(candle);
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let exchange = Exchange::new().unwrap();
        let start = Utc::now() - Duration::minutes(15);
        let symbol_minutes = SymbolMinutes::new("BTCUSDT", &15);
        let candles = exchange.candles(&symbol_minutes, &Some(start), &None);
        for candle in candles {
            iprintln!("{candle}");
        }
    }
}
