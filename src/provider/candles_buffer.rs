use crate::model::candle::Candle;

pub struct CandlesBuffer {
    pub candles: Vec<Candle>,
}

impl CandlesBuffer {
    pub fn new() -> Self {
        Self { candles: Vec::new() }
    }
}
