use crate::model::candle::Candle;
use crate::{
    config::definition::TacDefinition,
    technicals::{
        indicator::Indicator,
        technical::{TechnicalDefinition, TechnicalIndicators},
    },
};
use ifmt::iformat;
use log::info;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};

pub const MACD_IND: &str = "macd";

pub struct MacdTac<'a> {
    pub indicators: HashMap<String, Indicator<'a>>,
}

impl<'a> TechnicalDefinition<'a> for MacdTac<'a> {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![MACD_IND, "signal", "divergence"];
        TacDefinition::new(MACD_IND, &indicators)
    }
}
impl<'a> TechnicalIndicators<'a> for MacdTac<'a> {
    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(MACD_IND).unwrap()
    }

    fn indicators(&self) -> &HashMap<String, Indicator<'a>> {
        &self.indicators
    }
}

impl<'a> MacdTac<'a> {
    pub fn new(candles: &'a [&'a Candle], fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        let start = Instant::now();

        let mut macd = Indicator::new(MACD_IND, candles.len());
        let mut signal = Indicator::new("signal", candles.len());
        let mut divergence = Indicator::new("divergence", candles.len());
        let mut indicators = HashMap::new();

        // 34, 72, 17
        let mut macd_ta = Macd::new(fast_period, slow_period, signal_period).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let macd_result: (f64, f64, f64) = macd_ta.next(close).into();
            macd.push_serie(&candle.close_time, macd_result.0);
            signal.push_serie(&candle.close_time, macd_result.1);
            divergence.push_serie(&candle.close_time, macd_result.2);
        }

        indicators.insert(macd.name.clone(), macd);
        indicators.insert(signal.name.clone(), signal);
        indicators.insert(divergence.name.clone(), divergence);

        info!("{}", iformat!("Technicals {candles.len()}: {start.elapsed():?}"));

        MacdTac { indicators }
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
