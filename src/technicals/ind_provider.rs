use super::{
    ema_tac::{EmaTac, EMA_IND},
    ind_type::IndicatorType,
    macd::macd_tac::{MacdTac, MACD_DIV_IND, MACD_IND, MACD_SIG_IND},
    sma_tac::{SmaTac, SMA_IND},
    technical::TechnicalIndicators,
};
use crate::{application::candles_provider::CandlesProvider, technicals::indicator::Indicator};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct IndicatorProvider {
    mcads: HashMap<(usize, usize, usize), MacdTac>,
    tac_indicators: HashMap<(String, usize), anyhow::Result<Box<dyn TechnicalIndicators>>>, // <= to allow trait with different timelife
}

impl<'a, 'b> IndicatorProvider {
    pub fn new() -> Self {
        Self {
            mcads: HashMap::new(),
            tac_indicators: HashMap::new(),
        }
    }

    // TODO resolve this ugly Err(e) => return Err(anyhow!("{}", e)),
    fn tac_indicator(&mut self, candles_provider: Box<dyn CandlesProvider>, ind_name: &str, period: usize) -> anyhow::Result<&Indicator> {
        let result: &mut anyhow::Result<Box<dyn TechnicalIndicators>> = self.tac_indicators.entry((ind_name.to_string(), period)).or_insert_with(|| {
            let result: anyhow::Result<Box<dyn TechnicalIndicators>> = match ind_name {
                EMA_IND => Ok(Box::new(EmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators>), // <= cast box<struct> as box<trait>
                SMA_IND => Ok(Box::new(SmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators>),
                other => Err(anyhow!("Not found indicator {}!", other)),
            };
            result
        });
        let tac = match result {
            Ok(tac) => tac,
            Err(e) => return Err(anyhow!("{}", e)),
        };
        Ok(tac.main_indicator())
    }

    fn macd(
        &mut self,
        candles_provider: Box<dyn CandlesProvider>,
        ind_name: &str,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> anyhow::Result<&Indicator> {
        self.mcads
            .entry((fast_period, slow_period, signal_period))
            .or_insert_with(|| MacdTac::new(candles_provider, fast_period, slow_period, signal_period))
            .indicators
            .get(ind_name)
            .ok_or_else(|| -> anyhow::Error { anyhow!("Not found indicator {}!", ind_name) })
    }

    pub fn indicator(&mut self, candles_provider: Box<dyn CandlesProvider>, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => {
                self.macd(candles_provider, MACD_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::MacdSignal(fast_period, slow_period, signal_period) => {
                self.macd(candles_provider, MACD_SIG_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::MacdDivergence(fast_period, slow_period, signal_period) => {
                self.macd(candles_provider, MACD_DIV_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::Ema(period) => self.tac_indicator(candles_provider, EMA_IND, *period)?,
            IndicatorType::Sma(period) => self.tac_indicator(candles_provider, SMA_IND, *period)?,
            IndicatorType::TopBottom(period) => self.tac_indicator(candles_provider, "topbottom", *period)?,
        };
        Ok(ind)
    }
}
