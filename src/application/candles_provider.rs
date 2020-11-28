use crate::{
    config::selection::Selection, exchange::Exchange, model::candle::Candle, repository::Repository,
    utils::str_to_datetime,
};
use anyhow::Result;
use chrono::{Duration, Utc};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(repo: &'a Repository, exchange: &'a Exchange) -> Self {
        Self {
            exchange,
            repo,
            candles: Vec::new(),
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
