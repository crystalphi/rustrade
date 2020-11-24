// use crate::model::candle::Candle;

use super::indicator::Indicator;

pub trait Technical<'a> {
    // , T: Technical<'a, T>
    //fn new(candles: &'a [&'a Candle]) -> T;

    fn indicators(&'a self) -> &'a Vec<Indicator<'a>>;
}
