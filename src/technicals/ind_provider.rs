use super::{ema_tac::EmaTac, ind_type::IndicatorType, macd::macd_tac::MacdTac, pivots::PivotTac, sma_tac::SmaTac, technical::TechnicalIndicators};
use crate::{model::candle::Candle, technicals::indicator::Indicator};
use std::collections::HashMap;
pub struct IndicatorProvider<'a> {
    //indicators: HashMap<IndicatorType, &'a Indicator<'a>>,
    mcads: HashMap<(usize, usize, usize), MacdTac<'a>>,

    tac_indicators: HashMap<(String, usize), Box<dyn TechnicalIndicators<'a> + 'a>>, // <= to allow trait with different timelife
}

impl<'a> IndicatorProvider<'a> {
    pub fn new() -> Self {
        Self {
            //indicators: HashMap::new(),
            mcads: HashMap::new(),
            tac_indicators: HashMap::new(),
        }
    }

    // TODO resolve this ugly match inside or_inser_with
    fn tac_indicator(&'a mut self, candles: &'a [&Candle], ind_name: &str, period: usize) -> &Indicator {
        let tac = self.tac_indicators.entry((ind_name.to_string(), period)).or_insert_with(|| match ind_name {
            "ema" => Box::new(EmaTac::new(candles, period)) as Box<dyn TechnicalIndicators<'a>>, // <= cast box<struct> as box<trait>
            _/*"sma"*/ => Box::new(SmaTac::new(candles, period)) as Box<dyn TechnicalIndicators<'a>>,
        });
        tac.main_indicator()
    }

    fn macd(&'a mut self, candles: &'a [&Candle], ind_name: &str, fast_period: usize, slow_period: usize, signal_period: usize) -> &Indicator {
        self.mcads
            .entry((fast_period, slow_period, signal_period))
            .or_insert_with(|| MacdTac::new(candles, fast_period, slow_period, signal_period))
            .indicators
            .get(ind_name)
            .unwrap()
    }

    pub fn indicator(&'a mut self, candles: &'a [&Candle], i_type: &IndicatorType) -> anyhow::Result<&'a Indicator<'a>> {
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => self.macd(candles, "mcad", *fast_period, *slow_period, *signal_period),
            IndicatorType::Macd_signal(fast_period, slow_period, signal_period) => self.macd(candles, "signal", *fast_period, *slow_period, *signal_period),
            IndicatorType::Macd_divergence(fast_period, slow_period, signal_period) => {
                self.macd(candles, "divergence", *fast_period, *slow_period, *signal_period)
            }
            IndicatorType::Ema(period) => self.tac_indicator(candles, "ema", *period),
            IndicatorType::Sma(period) => self.tac_indicator(candles, "sma", *period),
            IndicatorType::Pivot(period) => self.tac_indicator(candles, "pivot", *period),
        };
        Ok(ind)
    }
}