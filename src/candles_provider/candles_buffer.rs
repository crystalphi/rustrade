use super::candles_provider::CandlesProviderTrait;

pub struct CandlesBuffer<'a> {
    candle_provider: Option<&'a dyn CandlesProviderTrait>,
}

impl<'a> CandlesBuffer<'a> {
    pub fn new(candle_provider: Option<&'a dyn CandlesProviderTrait>) -> Self {
        CandlesBuffer { candle_provider }
    }
}
