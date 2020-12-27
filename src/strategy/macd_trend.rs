use super::{trend_prov_factory::TrendProviderFactory, trend_provider::TrendProvider};
use crate::{model::candle::Candle, technicals::ind_provider::IndicatorProvider};

/// setup

/// transfer 1000 USD
/// buy 500 USD

pub struct MacdTrend<'a> {
    ind_provider: &'a IndicatorProvider<'a>,
}

impl<'a> MacdTrend<'a> {
    pub fn new(ind_provider: &'a IndicatorProvider<'a>) -> Self {
        Self { ind_provider }
    }
}

impl<'a> TrendProvider for MacdTrend<'a> {
    fn trend(&self, candle: &Candle) -> super::trend::Trend {
        todo!()
    }
}

impl<'a> TrendProviderFactory<'a, MacdTrend<'a>> for MacdTrend<'a> {
    fn create(ind_provider: &'a IndicatorProvider<'a>) -> MacdTrend<'a> {
        MacdTrend::new(ind_provider)
    }
}
