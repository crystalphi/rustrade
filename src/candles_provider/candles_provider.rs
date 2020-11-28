use crate::{config::symbol_minutes::SymbolMinutes, model::candle::Candle};

pub trait CandlesProviderTrait {
    fn candles(&self, symbol_minutes: &SymbolMinutes, start_time: &str, end_time: &str) -> Vec<Candle>;
}
