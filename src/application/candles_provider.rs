use crate::{
    config::selection::Selection, exchange::Exchange, model::candle::Candle, provider::candles_buffer::CandlesBuffer,
    repository::Repository, utils::str_to_datetime,
};
use anyhow::anyhow;
use anyhow::Result;
use chrono::{Duration, Utc};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
    current_start_time: Option<String>,
    current_end_time: Option<String>,
}

impl<'a> CandlesProvider<'a> {
    pub fn new(candles_buffer: &'a mut CandlesBuffer, repo: &'a Repository, exch: &'a Exchange) -> Self {
        Self {
            exchange: exch,
            repo,
            candles: Vec::new(),
            current_start_time: None,
            current_end_time: None,
        }
    }

    pub fn candles_selection(&mut self, selection: Selection) -> Result<Vec<Candle>> {
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

        let candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time, &end_time);

        // Candles
        // #symbol
        // #minutes
        //

        // Find candle in repo

        // Find candle in repo

        // self.candles = self
        //     .repo
        //     .candles_by_time(&selection.candles_selection.symbol_minutes, start_time, end_time)
        //     .unwrap_or_default();

        // let candles = self.candles.iter().collect();
        candles.ok_or(anyhow!("Not found"))
    }
}

fn diff_date_range() {}
