use super::{
    macd_trend::MacdTrend,
    trade_context_provider::TradeContextProvider,
    trader_register::{Position, TraderRegister},
    trend::Trend,
    trend_provider::TrendProvider,
};
use crate::{
    application::{app::Application, candles_provider::CandlesProvider, plot_selection::plot_selection},
    model::candle::Candle,
    tac_plotters::{indicator_plotter::PlotterIndicatorContext, trading_plotter::TradingPlotter},
    technicals::ind_provider::IndicatorProvider,
};
use chrono::{DateTime, Utc};
use ifmt::iformat;
use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::time::Instant;

pub struct Trader {
    trader_register: TraderRegister,
    trade_context_provider: TradeContextProvider,
    trend_provider: Box<dyn TrendProvider>,
    previous_trend: Option<Trend>,
}

impl<'a> Trader {
    pub fn new(trader_register: TraderRegister, trade_context_provider: TradeContextProvider, trend_provider: Box<dyn TrendProvider>) -> Self {
        Self {
            trader_register,
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
        if (&trend != previous_trend) && (&trend != self.trader_register.position().state()) {
            self.trader_register.register(now, trend.to_operation(), price);
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

    let position = Position::new_from_usd(dec!(1000));
    let trader_register = TraderRegister::new(position);

    let mcad_trend = MacdTrend::new();
    let mut trader = Trader::new(trader_register, trend_context_provider, Box::new(mcad_trend));

    let msg = format!("Running back test... candles.len {}", candles.len());
    info!("{}", msg);
    for i in 1..candles.len() {
        let candles_ref = &candles[0..=i];
        let c = candles_ref.last().unwrap();
        trader.check(candles_ref, c.close_time, c.close).unwrap();
    }

    let trades_ref = trader_register.trades().as_slice();

    let trading_plotter = TradingPlotter::new(trades_ref);

    let plotters = Vec::new();
    plotters.push(Box::new(trading_plotter) as Box<dyn PlotterIndicatorContext>);

    // TODO use plot_selection here
    plot_selection(selection.clone(), app.candles_provider.clone_provider(), plotters)?;

    info!("{}", iformat!("Finished backtest, elapsed: {start.elapsed():?}"));

    Ok(())
}
