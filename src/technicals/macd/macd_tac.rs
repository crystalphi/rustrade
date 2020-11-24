use crate::technicals::indicator::Indicator;
use crate::{model::candle::Candle, technicals::technical::Technical};
use ifmt::iprintln;
use rust_decimal::prelude::ToPrimitive;
use std::time::Instant;
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};
pub struct MacdCandle<'a> {
    pub candle: &'a Candle,
    // The MACD series proper
    pub macd: f64,
    // The "signal" or "average" series
    pub signal: f64,
    // The "divergence" series which is the difference between the two
    pub divergence: f64,
}

impl<'a> MacdCandle<'a> {
    pub fn new(candle: &'a Candle, macd: (f64, f64, f64)) -> Self {
        MacdCandle {
            candle,
            macd: macd.0,
            signal: macd.1,
            divergence: macd.2,
        }
    }
}

pub struct MacdTac<'a> {
    candles: &'a [&'a Candle],
    indicators: Vec<Box<dyn Indicator<'a>>>,
}

impl<'a> Technical<'a> for MacdTac<'a> {
    fn indicators(&'a self) -> &'a Vec<Box<dyn Indicator<'a>>> {
        &self.indicators
    }
}

impl<'a> MacdTac<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        MacdTac {
            candles,
            indicators: vec![],
        }
    }

    pub fn run(&self) -> Vec<MacdCandle> {
        let start = Instant::now();
        let mut technicals = Vec::new();
        let mut macd = Macd::new(34, 72, 17).unwrap();
        for candle in self.candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let macd_result = macd.next(close).into();

            let ta = MacdCandle::new(candle, macd_result);
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
