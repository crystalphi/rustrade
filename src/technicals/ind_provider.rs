use super::{ema_tac::EmaTac, ind_type::IndicatorType, macd::macd_tac::MacdTac, sma_tac::SmaTac, technical::Technical};
use crate::{model::candle::Candle, technicals::indicator::Indicator};
use anyhow::*;
use std::collections::{hash_map::Entry, HashMap};
pub struct IndicatorProvider<'a> {
    //indicators: HashMap<IndicatorType, &'a Indicator<'a>>,
    mcads: HashMap<(usize, usize, usize), MacdTac<'a>>,

    inds: HashMap<(String, usize), Box<dyn Technical<'a>>>,
}

impl<'a> IndicatorProvider<'a> {
    pub fn new() -> Self {
        Self {
            //indicators: HashMap::new(),
            mcads: HashMap::new(),
            inds: HashMap::new(),
        }
    }

    fn macd(&'a self, candles: &'a [&Candle], ind_name: &str, fast_period: usize, slow_period: usize, signal_period: usize) -> &Indicator {
        self.mcads
            .entry((fast_period, slow_period, signal_period))
            .or_insert_with(|| MacdTac::new(candles, fast_period, slow_period, signal_period))
            .indicators
            .get(ind_name)
            .unwrap()
    }

    pub fn indicator(&'a self, candles: &[&Candle], i_type: &IndicatorType) -> anyhow::Result<&'a Indicator<'a>> {
        //let ind = self.indicators.entry(*i_type).or_insert_with_key(|i_type|
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => self.macd(candles, "mcad", *fast_period, *slow_period, *signal_period),
            IndicatorType::Macd_signal(fast_period, slow_period, signal_period) => self.macd(candles, "signal", *fast_period, *slow_period, *signal_period),
            IndicatorType::Macd_divergence(fast_period, slow_period, signal_period) => {
                self.macd(candles, "divergence", *fast_period, *slow_period, *signal_period)
            }
            IndicatorType::Ema(period) => self
                .inds
                .entry(("ema".to_string(), *period))
                .or_insert_with(|| EmaTac::new(candles, *period).indicator()),
            IndicatorType::Sma(period) => self.indicators.entry(i_type).or_insert_with(|| SmaTac::new(candles, *period)),
            IndicatorType::Pivot(neighbors) => self.indicators.entry(i_type).or_insert_with(|| PivotTac::new(candles, *neighbors)),
        };
        //);

        Ok(ind)
    }
}
