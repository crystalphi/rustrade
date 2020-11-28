use super::candles_provider::CandlesProviderTrait;

pub struct CandlesResolver {
    parent: Option<Box<CandlesResolver>>,
    symbol: String,
    minutes: u32,
    start_time: Option<String>,
    end_time: Option<String>,
    candle_provider: Box<dyn CandlesProviderTrait>,
}

impl CandlesResolver {
    pub fn new(
        symbol: String,
        minutes: u32,
        start_time: Option<String>,
        end_time: Option<String>,
        candle_provider: Box<dyn CandlesProviderTrait>,
        parent: Option<Box<CandlesResolver>>,
    ) -> Self {
        CandlesResolver {
            parent,
            symbol,
            minutes,
            start_time,
            end_time,
            candle_provider,
        }
    }
}
