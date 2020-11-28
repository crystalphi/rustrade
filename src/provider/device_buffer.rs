use crate::{model::candle::Candle, repository::Repository};

use super::device_trait::DeviceTrait;

pub struct DeviceBuff {
    candles: Vec<Candle>,
}

impl DeviceBuff {
    pub fn new() -> Self {
        Self { candles: Vec::new() }
    }
}

impl Default for DeviceBuff {
    fn default() -> Self {
        DeviceBuff::new()
    }
}

impl DeviceTrait for DeviceBuff {
    fn candles(&mut self) -> Vec<crate::model::candle::Candle> {
        todo!()
    }

    fn insert_candles(&mut self, candles: &[&crate::model::candle::Candle]) {
        todo!()
    }
}
