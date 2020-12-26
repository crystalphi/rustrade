use crate::{model::candle::Candle, technicals::ind_provider::IndicatorProvider};

use super::trend::Trend;

pub trait TrendProvider {
    fn trend(&self, candle: &Candle) -> Trend;
}
