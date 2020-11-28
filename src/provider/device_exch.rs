use crate::exchange::Exchange;

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
    fn candles(&mut self) -> Vec<crate::model::candle::Candle> {
        todo!()
    }

    fn insert_candles(&mut self, candles: &[&crate::model::candle::Candle]) {
        todo!()
    }
}
