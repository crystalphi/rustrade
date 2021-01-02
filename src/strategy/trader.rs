use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::{macd_trend::MacdTrend, trade_context_provider::TradeContextProvider, trend::Trend, trend_provider::TrendProvider};
use crate::{application::app::Application, technicals::ind_provider::IndicatorProvider};

pub struct Trader<'a> {
    trade_context_provider: TradeContextProvider<'a>,
    trend_provider: Box<dyn TrendProvider<'a> + 'a>,
    previous_trend: Option<Trend>,
}

impl<'a> Trader<'a> {
    pub fn new(trade_context_provider: TradeContextProvider<'a>, trend_provider: Box<dyn TrendProvider<'a> + 'a>) -> Self {
        Self {
            trade_context_provider,
            trend_provider,
            previous_trend: None,
        }
    }

    pub fn check(&mut self, now: DateTime<Utc>, price: Decimal) -> anyhow::Result<()> {
        self.trade_context_provider.now_provider.set_now(now);

        let trend = self.trend_provider.trend(&mut self.trade_context_provider)?;
        let previous_trend = self.previous_trend.get_or_insert_with(|| trend.clone());
        if &trend != previous_trend {
            match trend {
                Trend::Bought => {
                    println!("{} Bought {}", now, price)
                }
                Trend::Sold => {
                    println!("{} Sold {}", now, price)
                }
            }
        }
        Ok(())
    }
}

pub fn run_trader_back_test(app: &mut Application) -> anyhow::Result<()> {
    let selection = &app.selection;
    let candles = app.candles_provider.candles_selection(&selection.candles_selection)?;

    let indicator_provider = IndicatorProvider::new();

    let trend_context_provider = TradeContextProvider::new(
        &selection.candles_selection.symbol_minutes.symbol,
        indicator_provider,
        &mut app.candles_provider,
    );

    let mcad_trend = MacdTrend::new();
    let mut trader = Trader::new(trend_context_provider, Box::new(mcad_trend));

    println!("Running back test...");
    for i in 1..candles.len() {
        let candles_ref = &candles[0..=i];
        for candle in candles_ref {
            trader.check(candle.close_time, candle.close)?;
        }
    }

    // Trader::run(&mcad_trend);
    Ok(())
}
