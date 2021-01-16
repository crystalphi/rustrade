use std::time::Instant;

use super::{macd_trend::MacdTrend, trade_context_provider::TradeContextProvider, trend::Trend, trend_provider::TrendProvider};
use crate::{
    application::{app::Application, candles_provider::CandlesProvider},
    model::candle::Candle,
    technicals::ind_provider::IndicatorProvider,
};
use chrono::{DateTime, Utc};
use ifmt::iformat;
use log::info;
use rust_decimal::Decimal;

pub struct Trader {
    trade_context_provider: TradeContextProvider,
    trend_provider: Box<dyn TrendProvider>,
    previous_trend: Option<Trend>,
}

impl<'a> Trader {
    pub fn new(trade_context_provider: TradeContextProvider, trend_provider: Box<dyn TrendProvider>) -> Self {
        Self {
            trade_context_provider,
            trend_provider,
            previous_trend: None,
        }
    }

    pub fn check(&'a mut self, candles: &[Candle], now: DateTime<Utc>, price: Decimal) -> anyhow::Result<()> {
        let trade_context_provider = &mut self.trade_context_provider;
        trade_context_provider.set_now(now);

        let trend_provider = &self.trend_provider;

        let trend = trend_provider.trend(trade_context_provider)?;

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
        self.previous_trend = Some(trend);
        Ok(())
    }
}

pub fn run_trader_back_test(app: &mut Application) -> anyhow::Result<()> {
    let start = Instant::now();
    info!("Initializing backtest...");

    let selection = &app.selection;
    app.candles_provider.set_candles_selection(selection.candles_selection.clone());
    let candles = app.candles_provider.candles()?;

    let indicator_provider = IndicatorProvider::new();

    let trend_context_provider = TradeContextProvider::new(
        &selection.candles_selection.symbol_minutes.symbol,
        indicator_provider,
        app.candles_provider.clone(),
    );

    let mcad_trend = MacdTrend::new();
    let mut trader = Trader::new(trend_context_provider, Box::new(mcad_trend));

    let msg = format!("Running back test... candles.len {}", candles.len());
    info!("{}", msg);
    for i in 1..candles.len() {
        let candles_ref = &candles[0..=i];
        let c = candles_ref.last().unwrap();
        trader.check(candles_ref, c.close_time, c.close).unwrap();
    }

    info!("{}", iformat!("Finished backtest, elapsed: {start.elapsed():?}"));

    Ok(())
}
