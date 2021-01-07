use super::{trade_context_provider::TradeContextProvider, trend::Trend};
use chrono::{DateTime, Utc};

pub trait TrendProvider {
    fn trend(&self, trend_context_provider: &mut TradeContextProvider, now: DateTime<Utc>) -> anyhow::Result<Trend>;
}
