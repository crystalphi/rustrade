use crate::model::candle::Candle;

use super::trend::Trend;

pub trait TrendProvider<'a> {
    fn trend(&mut self, candles: &'a [&Candle]) -> Trend;
}
