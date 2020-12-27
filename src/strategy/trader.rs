use crate::{application::app::Application, model::candle::Candle, technicals::ind_provider::IndicatorProvider};

use super::{macd_trend::MacdTrend, trend_provider::TrendProvider};

pub struct Trader<'a> {
    tend_provider: Box<dyn TrendProvider<'a> + 'a>,
}

impl<'a> Trader<'a> {
    pub fn new(tend_provider: Box<dyn TrendProvider<'a> + 'a>) -> Self {
        Self { tend_provider }
    }

    pub fn check(&'a self, candles: &'a [&Candle]) {
        let _trend = self.tend_provider.trend(candles);
    }
}

pub fn run_trader(app: &mut Application) -> anyhow::Result<()> {
    let selection = &app.selection;
    let candles = app.candles_provider.candles_selection(selection)?;
    let candles = candles.iter().collect::<Vec<_>>();
    let candles_ref = candles.as_slice();

    let mcad_trend = MacdTrend::new(IndicatorProvider::new());
    let trader = Trader::new(Box::new(mcad_trend));

    trader.check(candles_ref);

    //Trader::run(&mcad_trend);
    Ok(())
}
