use crate::{model::candle::Candle, repository::Repository};
use chrono::{Duration, Utc};
use ifmt::iprintln;
use rust_decimal::prelude::ToPrimitive;
use std::time::Instant;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub struct Technical<'a> {
    pub candle: &'a Candle,
    pub fast: f64,
    pub slow: f64,
    pub signal: f64,
}

impl<'a> Technical<'a> {
    fn new(candle: &'a Candle, macd: (f64, f64, f64)) -> Self {
        Technical {
            candle,
            fast: macd.0,
            slow: macd.1,
            signal: macd.2,
        }
    }
}

pub struct Analyzer<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> Analyzer<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        Analyzer { candles }
    }

    pub fn run(&self) -> Vec<Technical> {
        let start = Instant::now();
        let mut ema_9 = Ema::new(9).unwrap();
        let mut technicals = Vec::new();
        let mut macd = Macd::new(34, 72, 17).unwrap();
        for candle in self.candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let ta = Technical::new(candle, macd.next(close).into());
            technicals.push(ta);
        }
        iprintln!("Technicals {technicals.len()}: {start.elapsed():?}");
        technicals
    }
}

/*

    java:
    /home/vanius/Documents/java/TradeBot/src/main/java/br/com/vanius/tradebot/trader/TraderMACD.java
        MACDIndicator macd = new MACDIndicator(closePriceIndicator, 12, 26);
        EMAIndicator sma = new EMAIndicator(macd, 9);
        currentInd = currentMACD.subtract(currentSMA);

    rust:

                        // let ema_val = ema_9.next(&dt);

                        // 17,34,72
                        // let mut macd = Macd::new(3, 6, 4).unwrap();
                        // macd.next(&dt);

*/
