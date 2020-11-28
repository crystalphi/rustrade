use crate::{config::symbol_minutes::SymbolMinutes, model::candle::Candle, repository::Repository};

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesRepo<'a> {
    repo: &'a Repository,
}

impl<'a> CandlesRepo<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        CandlesRepo { repo }
    }
}

impl<'a> CandlesProviderTrait for CandlesRepo<'a> {
    fn candles(&self, symbol_minutes: &SymbolMinutes, start_time: &str, end_time: &str) -> Vec<Candle> {
        todo!()
    }
}
