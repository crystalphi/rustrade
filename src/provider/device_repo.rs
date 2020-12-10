use crate::{model::candle::Candle, repository::Repository};

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
    fn write(&mut self, candles: &[&Candle]) {
        todo!()
    }

    fn read_ref(
        &mut self,
        selection: &crate::config::candles_selection::CandlesSelection,
    ) -> (crate::config::candles_selection::DatesSelection, Vec<&Candle>) {
        todo!()
    }

    fn read_own(
        &mut self,
        selection: &crate::config::candles_selection::CandlesSelection,
    ) -> (crate::config::candles_selection::DatesSelection, Vec<Candle>) {
        todo!()
    }

    fn on_new_from_child(&mut self, candles: Vec<Candle>) {
        todo!()
    }
}
