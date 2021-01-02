use chrono::{DateTime, Utc};

use super::{trade_context_provider::TradeContextProvider, trend::Trend};

pub trait TrendProvider<'a> {
    fn trend(&mut self, trend_context_provider: TradeContextProvider<'a>) -> anyhow::Result<Trend>;
}
