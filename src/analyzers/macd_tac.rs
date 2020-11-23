use crate::model::candle::Candle;
use ifmt::iprintln;
use rust_decimal::prelude::ToPrimitive;
use std::time::Instant;
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};

pub struct MacdCandle<'a> {
    pub candle: &'a Candle,
    // The MACD series proper
    pub macd: f64,
    // The "signal" or "average" series
    pub fast: f64,
    // The "divergence" series which is the difference between the two
    pub divergence: f64,
}

impl<'a> MacdCandle<'a> {
    pub fn new(candle: &'a Candle, macd: (f64, f64, f64)) -> Self {
        MacdCandle {
            candle,
            macd: macd.0,
            fast: macd.1,
            divergence: macd.2,
        }
    }
}

pub struct MacdTac<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> MacdTac<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        MacdTac { candles }
    }

    pub fn run(&self) -> Vec<MacdCandle> {
        let start = Instant::now();
        let mut technicals = Vec::new();
        let mut macd = Macd::new(34, 72, 17).unwrap();
        for candle in self.candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let ta = MacdCandle::new(candle, macd.next(close).into());
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
