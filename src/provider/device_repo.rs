use crate::repository::Repository;

use super::device_trait::DeviceTrait;

pub struct DeviceRepo<'a> {
    repo: &'a Repository,
}

impl<'a> DeviceRepo<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
    }
}

impl<'a> DeviceTrait for DeviceRepo<'a> {
    fn candles(&mut self) -> Vec<crate::model::candle::Candle> {
        todo!()
    }

    fn insert_candles(&mut self, candles: &[&crate::model::candle::Candle]) {
        todo!()
    }
}
