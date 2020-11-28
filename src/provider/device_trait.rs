use crate::model::candle::Candle;

pub trait DeviceTrait {
    fn candles(&mut self) -> Vec<Candle>;
    fn insert_candles(&mut self, candles: &[&Candle]);
}
