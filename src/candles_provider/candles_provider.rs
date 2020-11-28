use crate::model::candle::Candle;

pub trait CandlesProviderTrait {
    fn candles(&self, symbol: &str, minutes: &u32, start_time: &str, end_time: &str) -> Vec<Candle>;
}
