use crate::{config::symbol_minutes::SymbolMinutes, model::candle::Candle};

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesBuffer {}

impl CandlesBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> CandlesProviderTrait for CandlesBuffer {
    fn candles(&self, symbol_minutes: &SymbolMinutes, start_time: &str, end_time: &str) -> Vec<Candle> {
        todo!()
    }
}
