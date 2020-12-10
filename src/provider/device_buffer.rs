use crate::{
    config::candles_selection::CandlesSelection, config::candles_selection::DatesSelection, model::candle::Candle,
};

use super::{candles_buffer::CandlesBuffer, device_trait::DeviceTrait};

pub struct DeviceBuff<'a> {
    candles_buffer: &'a CandlesBuffer,
}

impl<'a> DeviceBuff<'a> {
    pub fn new(candles_buffer: &'a CandlesBuffer) -> Self {
        Self { candles_buffer }
    }
}

impl<'a> DeviceTrait for DeviceBuff<'a> {
    fn write(&mut self, candles: &[&Candle]) {
        todo!()
    }

    fn read_ref(&mut self, selection: &CandlesSelection) -> (DatesSelection, Vec<&Candle>) {
        todo!()
    }

    fn read_own(&mut self, selection: &CandlesSelection) -> (DatesSelection, Vec<Candle>) {
        todo!()
    }

    fn on_new_from_child(&mut self, candles: Vec<Candle>) {
        todo!()
    }
}
