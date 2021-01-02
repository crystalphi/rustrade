use super::{trade_context_provider::TradeContextProvider, trend::Trend};

pub trait TrendProvider<'a> {
    fn trend(&mut self, trend_context_provider: &'a mut TradeContextProvider<'a>) -> anyhow::Result<Trend>;
}
