use crate::{
    config::candles_selection::CandlesSelection, config::candles_selection::DatesSelection, model::candle::Candle,
};

use self::{candles_buffer::CandlesBuffer, device_trait::DeviceTrait};

pub mod candles_buffer;
pub mod device_buffer;
pub mod device_exch;
pub mod device_repo;
pub mod device_trait;

pub struct Provider<'a> {
    candles_buffer: &'a CandlesBuffer,
    device: Box<dyn DeviceTrait + 'a>,
    parent: Option<Box<Provider<'a>>>,
}

impl<'a> Provider<'a> {
    pub fn new(
        candles_buffer: &'a CandlesBuffer,
        device: Box<dyn DeviceTrait + 'a>,
        parent: Option<Box<Provider<'a>>>,
    ) -> Self {
        Self {
            candles_buffer,
            device,
            parent,
        }
    }

    pub fn candles(&mut self, selection: CandlesSelection) -> Vec<&Candle> {
        let (dates_in_buffer, mut candles_in_buffer) = self.device.read_ref(&selection);
        let diff = self.diff(&selection, &dates_in_buffer);

        if diff.start_time.is_some() || diff.start_time.is_some() {
            if let Some(parent) = self.parent.as_mut() {
                let mut candles_parent = parent.device.read_own(&selection);
                // Insert parent candles into returning candles
                self.device
                    .write(candles_parent.1.iter().collect::<Vec<_>>().as_slice());
                candles_parent.1.iter().for_each(|c| candles_in_buffer.push(c));
                self.candles_buffer.candles.append(&mut candles_parent.1);
            }
        }
        candles_in_buffer
    }

    pub fn diff(&mut self, selection: &CandlesSelection, dates_in_buffer: &DatesSelection) -> CandlesSelection {
        // deve comparar a data start e end corrente
        todo!()
    }
}
