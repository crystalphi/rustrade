use crate::{exchange::Exchange, model::candle::Candle};

use super::device_trait::DeviceTrait;

pub struct DeviceExch<'a> {
    exch: &'a Exchange,
}

impl<'a> DeviceExch<'a> {
    pub fn new(exch: &'a Exchange) -> Self {
        Self { exch }
    }
}

impl<'a> DeviceTrait for DeviceExch<'a> {
    fn read(
        &mut self,
        selection: &crate::config::candles_selection::CandlesSelection,
    ) -> (crate::config::candles_selection::DatesSelection, Vec<Candle>) {
        todo!()
    }

    fn write(&mut self, candles: &[&Candle]) {
        todo!()
    }
}
