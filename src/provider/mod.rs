use crate::{
    config::candles_selection::CandlesSelection, config::candles_selection::DatesSelection, model::candle::Candle,
};

use self::device_trait::DeviceTrait;

pub mod candles_buffer;
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

    pub fn candles(&mut self, selection: CandlesSelection) -> Vec<&Candle> {
        // let (dates_in_buffer, candles_in_buffer) = &self.device.read_ref(&selection);
        // let diff = diff(&selection, &dates_in_buffer);

        // if diff.start_time.is_some() || diff.start_time.is_some() {
        //     if let Some(parent) = self.parent.as_mut() {
        //         let mut candles_parent = parent.device.read_own(&selection);

        //         // Insert parent candles into returning candles
        //         self.device
        //             .write(candles_parent.1.iter().collect::<Vec<_>>().as_slice());
        //         candles_parent.1.iter().for_each(|c| candles_in_buffer.push(c));

        //         &mut self.candles_buffer.candles.append(&mut candles_parent.1);
        //     }
        // }
        // candles_in_buffer.clone()
        todo!()
    }
}

pub fn diff(selection: &CandlesSelection, dates_in_buffer: &DatesSelection) -> CandlesSelection {
    // deve comparar a data start e end corrente
    todo!()
}
