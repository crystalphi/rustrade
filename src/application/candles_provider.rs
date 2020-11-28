use crate::{
    config::selection::Selection,
    exchange::Exchange,
    model::candle::Candle,
    provider::{device_buffer::DeviceBuff, device_exch::DeviceExch, device_repo::DeviceRepo, Provider},
    repository::Repository,
    utils::str_to_datetime,
};
use anyhow::Result;
use chrono::{Duration, Utc};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
    current_start_time: Option<String>,
    current_end_time: Option<String>,
    provider_exchange: Provider<'a>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(repo: &'a Repository, exch: &'a Exchange) -> Self {
        let provider_exch = Provider::new(Box::new(DeviceExch::new(exch)), None);
        let provider_repo = Provider::new(Box::new(DeviceRepo::new(repo)), Some(Box::new(provider_exch)));
        let provider_buff = Provider::new(Box::new(DeviceBuff::new()), Some(Box::new(provider_repo)));

        Self {
            provider_exchange: provider_buff,
            exchange: exch,
            repo,
            candles: Vec::new(),
            current_start_time: None,
            current_end_time: None,
        }
    }

    pub fn candles_selection(&mut self, selection: Selection) -> Result<Vec<&Candle>> {
        let start_time = &selection
            .candles_selection
            .start_time
            .map(|s| str_to_datetime(&s))
            .unwrap_or_else(|| Utc::now() - Duration::days(180));

        let end_time = &selection
            .candles_selection
            .end_time
            .map(|s| str_to_datetime(&s))
            .unwrap_or_else(Utc::now);

        self.candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, start_time, end_time)
            .unwrap_or_default();

        let candles = self.candles.iter().collect();
        Ok(candles)
    }
}
