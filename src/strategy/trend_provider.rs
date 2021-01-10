use super::{trade_context_provider::TradeContextProvider, trend::Trend};

pub trait TrendProvider {
    fn trend(&self, trend_context_provider: &mut TradeContextProvider) -> anyhow::Result<Trend>;
}
