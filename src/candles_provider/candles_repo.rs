use crate::{model::candle::Candle, repository::Repository};

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesRepo<'a> {
    candle_provider: Option<&'a dyn CandlesProviderTrait>,
    repo: &'a Repository,
}

impl<'a> CandlesRepo<'a> {
    pub fn new(candle_provider: Option<&'a dyn CandlesProviderTrait>, repo: &'a Repository) -> Self {
        CandlesRepo { candle_provider, repo }
    }
}

impl<'a> CandlesProviderTrait for CandlesRepo<'a> {
    fn candles(&self, symbol: &str, minutes: &u32, start_time: &str, end_time: &str) -> Vec<Candle> {
        todo!()
    }
}
