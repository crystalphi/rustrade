use super::technical::{TechnicalDefinition, TechnicalIndicators};
use crate::application::candles_provider::CandlesProvider;
use crate::{config::definition::TacDefinition, technicals::indicator::Indicator};
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub const EMA_IND: &str = "ema";

#[derive(Clone)]
pub struct EmaTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for EmaTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![EMA_IND];
        TacDefinition::new(EMA_IND, &indicators)
    }
}

impl TechnicalIndicators for EmaTac {
    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(EMA_IND).unwrap()
    }
}

impl<'a> EmaTac {
    // default period is 34
    pub fn new(mut candles_provider: Box<dyn CandlesProvider>, period: usize) -> Self {
        let candles = candles_provider.candles().unwrap();

        let mut ema = Indicator::new(EMA_IND, candles.len());
        let mut indicators = HashMap::new();

        let mut ema_ta = Ema::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let ema_result = ema_ta.next(close);
            ema.push_serie(candle.close_time, ema_result);
        }

        indicators.insert(ema.name.clone(), ema);

        EmaTac { indicators }
    }

    pub fn _indicator(&self) -> &Indicator {
        self.indicators.get(EMA_IND).unwrap()
    }
}
