use crate::{
    config::candles_selection::CandlesSelection, config::candles_selection::DatesSelection, model::candle::Candle,
};

pub trait DeviceTrait {
    fn read_ref(&mut self, selection: &CandlesSelection) -> (DatesSelection, Vec<&Candle>);
    fn read_own(&mut self, selection: &CandlesSelection) -> (DatesSelection, Vec<Candle>);
    fn write(&mut self, candles: &[&Candle]);
    fn on_new_from_child(&mut self, candles: Vec<Candle>);
}

