use crate::config::candles_selection::CandlesSelection;

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesResolver {
    parent: Option<Box<CandlesResolver>>,
    candles_selection: CandlesSelection,
    candle_provider: Box<dyn CandlesProviderTrait + 'static>,
}

impl CandlesResolver {
    pub fn new(
        candles_selection: &CandlesSelection,
        candle_provider: Box<dyn CandlesProviderTrait>,
        parent: Option<Box<CandlesResolver>>,
    ) -> Self {
        Self {
            candles_selection: candles_selection.clone(),
            parent,
            candle_provider,
        }
    }
}
