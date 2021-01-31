use super::{
    ema_tac::{EmaTac, EMA_IND},
    ind_type::IndicatorType,
    macd::macd_tac::{MacdTac, MACD_DIV_IND, MACD_IND, MACD_SIG_IND},
    sma_tac::{SmaTac, SMA_IND},
    technical::TechnicalIndicators,
};
use crate::{application::candles_provider::CandlesProvider, technicals::indicator::Indicator};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct IndicatorProvider {
    mcads_opt: Option<(DateTime<Utc>, usize, usize, usize, MacdTac)>,
    tac_indicators: HashMap<(String, usize), anyhow::Result<Box<dyn TechnicalIndicators + Send + Sync>>>, // <= to allow trait with different timelife
}

impl Clone for IndicatorProvider {
    fn clone(&self) -> Self {
        Self {
            mcads_opt: self.mcads_opt.clone(),
            tac_indicators: HashMap::new(),
        }
    }
}

impl IndicatorProvider {
    pub fn new() -> Self {
        Self {
            mcads_opt: None,
            tac_indicators: HashMap::new(),
        }
    }
    fn tac_indicator(&mut self, candles_provider: Box<dyn CandlesProvider>, ind_name: &str, period: usize) -> anyhow::Result<&Indicator> {
        self.tac_indicators.clear();
        // TODO I shouldn't store Indicator cache, or use "now" like a key
        let result: &mut anyhow::Result<Box<dyn TechnicalIndicators + Send + Sync>> =
            self.tac_indicators.entry((ind_name.to_string(), period)).or_insert_with(|| {
                let result: anyhow::Result<Box<dyn TechnicalIndicators + Send + Sync>> = match ind_name {
                    EMA_IND => Ok(Box::new(EmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators + Send + Sync>), // <= cast box<struct> as box<trait>
                    SMA_IND => Ok(Box::new(SmaTac::new(candles_provider, period)) as Box<dyn TechnicalIndicators + Send + Sync>),
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
        now: DateTime<Utc>,
        candles_provider: Box<dyn CandlesProvider>,
        ind_name: &str,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> anyhow::Result<&Indicator> {
        self.mcads_opt = self
            .mcads_opt
            .take()
            .filter(|e| e.0 == now && e.1 == fast_period && e.2 == slow_period && e.3 == signal_period);
        let macd = self.mcads_opt.get_or_insert_with(|| {
            (
                now,
                fast_period,
                slow_period,
                signal_period,
                MacdTac::new(candles_provider, fast_period, slow_period, signal_period),
            )
        });
        let result = macd
            .4
            .indicators
            .get(ind_name)
            .ok_or_else(|| -> anyhow::Error { anyhow!("Not found indicator {}!", ind_name) });
        result
    }

    pub fn indicator(&mut self, now: DateTime<Utc>, candles_provider: Box<dyn CandlesProvider>, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => {
                self.macd(now, candles_provider, MACD_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::MacdSignal(fast_period, slow_period, signal_period) => {
                self.macd(now, candles_provider, MACD_SIG_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::MacdDivergence(fast_period, slow_period, signal_period) => {
                self.macd(now, candles_provider, MACD_DIV_IND, *fast_period, *slow_period, *signal_period)?
            }
            IndicatorType::Ema(period) => self.tac_indicator(candles_provider, EMA_IND, *period)?,
            IndicatorType::Sma(period) => self.tac_indicator(candles_provider, SMA_IND, *period)?,
            IndicatorType::TopBottom(period) => self.tac_indicator(candles_provider, "topbottom", *period)?,
        };
        Ok(ind)
    }
}
