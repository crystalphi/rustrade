use crate::{model::candle::Candle, technicals::ind_provider::IndicatorProvider};

use super::{macd_trend::MacdTrend, trend_prov_factory::TrendProviderFactory, trend_provider::TrendProvider};

pub struct Trader {}

impl<'a> Trader {
    pub fn run<S, T>(indicator_provider: &'a IndicatorProvider<'a>) -> T
    where
        T: TrendProvider,
        S: TrendProviderFactory<'a, T>,
        //TrendProviderFactory<'a, T: TrendProvider + 'a>
    {
        S::create(&indicator_provider)
    }
}

pub fn run_trader(candles: &[&Candle]) {
    let indicator_provider = IndicatorProvider::new();

    let trader: MacdTrend = Trader::run(&indicator_provider);
}
