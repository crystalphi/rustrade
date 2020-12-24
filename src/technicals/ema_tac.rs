use crate::{config::definition::TacDefinition, technicals::indicator::Indicator};
use crate::{model::candle::Candle, technicals::technical::Technical};
use ifmt::iformat;
use log::info;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub struct EmaTac<'a> {
    pub indicators: HashMap<String, Indicator<'a>>,
}

impl<'a> Technical for EmaTac<'a> {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec!["ema"];
        TacDefinition::new("ema", &indicators)
    }
}

impl<'a> EmaTac<'a> {
    // default period is 34
    pub fn new(candles: &'a [&'a Candle], period: usize) -> Self {
        let start = Instant::now();

        let mut ema = Indicator::new("ema");
        let mut indicators = HashMap::new();

        let mut ema_ta = Ema::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let ema_result = ema_ta.next(close);
            ema.push_serie(&candle.close_time, ema_result);
        }

        indicators.insert(ema.name.clone(), ema);

        info!("{}", iformat!("Technicals {candles.len()}: {start.elapsed():?}"));

        EmaTac { indicators }
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
