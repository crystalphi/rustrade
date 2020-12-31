use crate::{application::app::Application, model::candle::Candle, technicals::ind_provider::IndicatorProvider};

use super::{macd_trend::MacdTrend, trend_provider::TrendProvider};

pub struct Trader<'a> {
    tend_provider: Box<dyn TrendProvider<'a> + 'a>,
}

impl<'a> Trader<'a> {
    pub fn new(tend_provider: Box<dyn TrendProvider<'a> + 'a>) -> Self {
        Self { tend_provider }
    }

    pub fn check(&mut self, candles: &'a [&Candle]) {
        self.tend_provider.trend(candles);
    }
}

pub struct TraderS {}

impl<'a, 'b> TraderS {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&mut self, candles: &'b [&Candle]) {
        //self.tend_provider.trend(candles);
    }
}

pub fn run_trader(mut app: Application) -> anyhow::Result<()> {
    let selection = &app.selection;
    let candles = app.candles_provider.candles_selection(selection)?;
    let candles = candles.iter().collect::<Vec<_>>();
    let candles_ref = candles.as_slice();

    let indicator_provider = IndicatorProvider::new();

    let mut trader_s = TraderS::new();
    trader_s.check(candles_ref);

    let mcad_trend = MacdTrend::new(indicator_provider);
    let mut trader = Trader::new(Box::new(mcad_trend));

    trader.check(candles_ref);

    // Trader::run(&mcad_trend);
    Ok(())
}
