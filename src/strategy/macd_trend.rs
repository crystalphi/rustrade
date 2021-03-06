use super::{trade_context_provider::TradeContextProvider, trend::Trend, trend_provider::TrendProvider};
use crate::technicals::ind_type::IndicatorType;
use log::debug;

/// setup
/// transfer 1000 USD
/// buy 500 USD

pub struct MacdTrend {}

impl MacdTrend {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> TrendProvider for MacdTrend {
    fn trend(&self, trend_context_provider: &mut TradeContextProvider) -> anyhow::Result<Trend> {
        let mcad = trend_context_provider.indicator(15, &IndicatorType::Macd(34, 72, 17))?.value()?;
        let mcad_signal = trend_context_provider.indicator(15, &IndicatorType::MacdSignal(34, 72, 17))?.value()?;
        //let _mcad_divergence = trend_context_provider.indicator(15, &IndicatorType::MacdDivergence(34, 72, 17))?.value()?;
        let trend = if mcad > mcad_signal { Trend::Bought } else { Trend::Sold };

        debug!("trend: {:?} {} > {}", trend_context_provider.now(), mcad, mcad_signal);
        Ok(trend)
    }
}
