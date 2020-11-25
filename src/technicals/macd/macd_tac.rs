use crate::technicals::indicator::Indicator;
use crate::{model::candle::Candle, technicals::technical::Technical};
use ifmt::iprintln;
use rust_decimal::prelude::ToPrimitive;
use std::time::Instant;
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};

pub struct MacdTac<'a> {
    candles: &'a [&'a Candle],
    pub macd: Indicator<'a>,
    pub signal: Indicator<'a>,
    pub divergence: Indicator<'a>,
}

impl<'a> Technical<'a> for MacdTac<'a> {
    fn indicators(&'a self) -> Vec<&'a Indicator<'a>> {
        vec![&self.macd, &self.signal, &self.divergence]
    }
}

impl<'a> MacdTac<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        let mut r = MacdTac {
            candles,
            macd: Indicator::new("macd"),
            signal: Indicator::new("signal"),
            divergence: Indicator::new("divergence"),
        };
        r.run();
        r
    }

    fn run(&self) {
        let start = Instant::now();

        let mut macd_id = Indicator::new("macd");
        let mut signal = Indicator::new("signal");
        let mut divergence = Indicator::new("divergence");

        let mut macd = Macd::new(34, 72, 17).unwrap();
        for candle in self.candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let macd_result: (f64, f64, f64) = macd.next(close).into();
            macd_id.push_serie(&candle.close_time, macd_result.0);
            signal.push_serie(&candle.close_time, macd_result.1);
            divergence.push_serie(&candle.close_time, macd_result.2);
        }
        iprintln!("Technicals {self.candles.len()}: {start.elapsed():?}");
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
