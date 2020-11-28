use crate::{config::candles_selection::CandlesSelection, model::candle::Candle};

use self::device_trait::DeviceTrait;

pub mod device_buffer;
pub mod device_exch;
pub mod device_repo;
pub mod device_trait;

pub struct Provider<'a> {
    device: Box<dyn DeviceTrait + 'a>,
    parent: Option<Box<Provider<'a>>>,
}

impl<'a> Provider<'a> {
    pub fn new(device: Box<dyn DeviceTrait + 'a>, parent: Option<Box<Provider<'a>>>) -> Self {
        Self { device, parent }
    }

    pub fn candles(&mut self, selection: CandlesSelection) -> Vec<Candle> {
        let mut candles = self.device.candles();
        let diff = self.diff(&selection, &candles.iter().collect::<Vec<_>>().as_slice());
        if diff.start_time.is_some() || diff.start_time.is_some() {
            if let Some(parent) = self.parent.as_mut() {
                let mut candles_parent = parent.candles(diff);
                // Inset parent candles into returning candles
                self.device
                    .insert_candles(candles_parent.iter().collect::<Vec<_>>().as_slice());
                candles.append(&mut candles_parent);
            }
        }
        candles
    }

    pub fn diff(&mut self, selection: &CandlesSelection, candle: &[&Candle]) -> CandlesSelection {
        // deve comparar a data start e end corrente
        todo!()
    }
}
