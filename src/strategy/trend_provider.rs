use crate::model::candle::Candle;

use super::trend::Trend;

pub trait TrendProvider<'a> {
    fn trend(&'a self, candles: &'a [&Candle]) -> Trend;
}
