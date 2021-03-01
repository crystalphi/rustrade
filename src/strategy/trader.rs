use super::{trade_context_provider::TradeContextProvider, trader_register::Trade, trend::Trend, trend_provider::TrendProvider};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Trader {
    trade_context_provider: TradeContextProvider,
    trend_provider: Box<dyn TrendProvider + Send + Sync>,
    previous_trend: Option<Trend>,
    trades: Vec<Trade>,
}

impl<'a> Trader {
    pub fn new(trade_context_provider: TradeContextProvider, trend_provider: Box<dyn TrendProvider + Send + Sync>) -> Self {
        Self {
            trade_context_provider,
            trend_provider,
            previous_trend: None,
            trades: Vec::new(),
        }
    }

    pub fn check(&'a mut self, now: DateTime<Utc>, price: Decimal) -> anyhow::Result<()> {
        let trade_context_provider = &mut self.trade_context_provider;
        trade_context_provider.set_now(now);

        let trend_provider = &self.trend_provider;

        let trend = trend_provider.trend(trade_context_provider)?;

        let previous_trend = self.previous_trend.get_or_insert_with(|| trend.clone());

        if &trend != previous_trend {
            let trade = Trade::new(trend.to_operation(), now, price);
            self.trades.push(trade);
        }
        self.previous_trend = Some(trend);
        Ok(())
    }

    pub fn trades(&self) -> Vec<Trade> {
        self.trades.clone()
    }
}
