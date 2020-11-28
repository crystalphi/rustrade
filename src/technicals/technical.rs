// use crate::model::candle::Candle;

use crate::config::definition::TacDefinition;

use super::indicator::Indicator;

pub trait Technical {
    // , T: Technical<'a, T>
    //fn new(candles: &'a [&'a Candle]) -> T;

    //fn indicators(&'a self) -> Vec<&'a Indicator<'a>>;

    fn definition() -> TacDefinition;
}
