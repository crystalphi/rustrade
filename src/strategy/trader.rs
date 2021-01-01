use super::{macd_trend::MacdTrend, trend::Trend, trend_provider::TrendProvider};
use crate::{application::app::Application, model::candle::Candle, technicals::ind_provider::IndicatorProvider};

pub struct Trader<'a> {
    tend_provider: Box<dyn TrendProvider<'a> + 'a>,
    previous_trend: Option<Trend>,
}

impl<'a> Trader<'a> {
    pub fn new(tend_provider: Box<dyn TrendProvider<'a> + 'a>) -> Self {
        Self {
            tend_provider,
            previous_trend: None,
        }
    }

    pub fn check(&mut self, candles: &'a [&Candle]) -> anyhow::Result<()> {
        let candle = candles.last().unwrap();
        let trend = self.tend_provider.trend(candles)?;
        let previous_trend = self.previous_trend.get_or_insert_with(|| trend.clone());
        if &trend != previous_trend {
            match trend {
                Trend::Bought => {
                    println!("{} Bought {}", candle.close_time, candle.close)
                }
                Trend::Sold => {
                    println!("{} Sold {}", candle.close_time, candle.close)
                }
            }
        }
        Ok(())
    }
}

pub fn run_trader_back_test(app: &mut Application) -> anyhow::Result<()> {
    let selection = &app.selection;
    let candles = app.candles_provider.candles_selection(selection)?;
    let candles = candles.iter().collect::<Vec<_>>();

    let indicator_provider = IndicatorProvider::new();

    let mcad_trend = MacdTrend::new(indicator_provider);
    let mut trader = Trader::new(Box::new(mcad_trend));

    println!("Running back test...");
    for i in 1..candles.len() {
        let candles_ref = &candles[0..=i];
        trader.check(candles_ref)?;
    }

    // Trader::run(&mcad_trend);
    Ok(())
}
