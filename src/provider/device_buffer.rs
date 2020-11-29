use crate::{config::candles_selection::CandlesSelection, model::candle::Candle, config::candles_selection::DatesSelection};

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
    fn read(&mut self, selection: &CandlesSelection) -> (DatesSelection, Vec<Candle>) {
        todo!()
    }

    fn write(&mut self, candles: &[&Candle]) {
        todo!()
    }
}
