use crate::model::candle::Candle;

use super::trend::Trend;

pub trait TrendProvider {
    fn trend(&self, candle: &Candle) -> Trend;
}
