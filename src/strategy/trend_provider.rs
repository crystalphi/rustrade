use super::trend::Trend;
use crate::model::candle::Candle;

pub trait TrendProvider<'a> {
    fn trend(&mut self, candles: &'a [&Candle]) -> anyhow::Result<Trend>;
}
