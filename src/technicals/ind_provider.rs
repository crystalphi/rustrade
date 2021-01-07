use super::{ema_tac::EmaTac, ind_type::IndicatorType, macd::macd_tac::MacdTac, sma_tac::SmaTac, technical::TechnicalIndicators};
use crate::{application::candles_provider::CandlesProvider, technicals::indicator::Indicator};
use std::collections::HashMap;

pub struct IndicatorProvider {
    mcads: HashMap<(usize, usize, usize), MacdTac>,
    tac_indicators: HashMap<(String, usize), Box<dyn TechnicalIndicators>>, // <= to allow trait with different timelife
}

impl<'a, 'b> IndicatorProvider {
    pub fn new() -> Self {
        Self {
            mcads: HashMap::new(),
            tac_indicators: HashMap::new(),
        }
    }

    // TODO resolve this ugly match inside or_inser_with
    fn tac_indicator(&mut self, candles_provider: Box<dyn CandlesProvider>, ind_name: &str, period: usize) -> &Indicator {
        let tac = self.tac_indicators.entry((ind_name.to_string(), period)).or_insert_with(|| match ind_name {
            "ema" => Box::new(EmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators>, // <= cast box<struct> as box<trait>
            _/*"sma"*/ => Box::new(SmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators>,
        });
        tac.main_indicator()
    }

    fn macd(&mut self, candles_provider: Box<dyn CandlesProvider>, ind_name: &str, fast_period: usize, slow_period: usize, signal_period: usize) -> &Indicator {
        self.mcads
            .entry((fast_period, slow_period, signal_period))
            .or_insert_with(|| MacdTac::new(candles_provider, fast_period, slow_period, signal_period))
            .indicators
            .get(ind_name)
            .unwrap()
    }

    pub fn indicator(&mut self, candles_provider: Box<dyn CandlesProvider>, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => self.macd(candles_provider, "mcad", *fast_period, *slow_period, *signal_period),
            IndicatorType::MacdSignal(fast_period, slow_period, signal_period) => {
                self.macd(candles_provider, "signal", *fast_period, *slow_period, *signal_period)
            }
            IndicatorType::MacdDivergence(fast_period, slow_period, signal_period) => {
                self.macd(candles_provider, "divergence", *fast_period, *slow_period, *signal_period)
            }
            IndicatorType::Ema(period) => self.tac_indicator(candles_provider, "ema", *period),
            IndicatorType::Sma(period) => self.tac_indicator(candles_provider, "sma", *period),
            IndicatorType::TopBottom(period) => self.tac_indicator(candles_provider, "topbottom", *period),
        };
        Ok(ind)
    }
}
