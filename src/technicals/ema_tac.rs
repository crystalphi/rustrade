use super::technical::{TechnicalDefinition, TechnicalIndicators};
use crate::model::candle::Candle;
use crate::{config::definition::TacDefinition, technicals::indicator::Indicator};
use ifmt::iformat;
use log::info;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub struct EmaTac<'a> {
    pub indicators: HashMap<String, Indicator<'a>>,
}

impl<'a> TechnicalDefinition<'a> for EmaTac<'a> {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec!["ema"];
        TacDefinition::new("ema", &indicators)
    }
}

impl<'a> TechnicalIndicators<'a> for EmaTac<'a> {
    fn indicators(&self) -> &HashMap<String, Indicator<'a>> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get("ema").unwrap()
    }
}

impl<'a> EmaTac<'a> {
    // default period is 34
    pub fn new(candles: &'a [&'a Candle], period: usize) -> Self {
        let start = Instant::now();

        let mut ema = Indicator::new("ema", candles.len());
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

    pub fn indicator(&self) -> &Indicator {
        self.indicators.get("ema").unwrap()
    }
}
