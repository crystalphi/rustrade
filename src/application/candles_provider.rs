use crate::{
    config::selection::Selection, exchange::Exchange, model::candle::Candle, model::candles_result::CandlesResult,
    provider::candles_buffer::CandlesBuffer, repository::Repository,
};
use anyhow::anyhow;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

pub struct CandlesProvider<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles: Vec<Candle>,
    current_start_time: Option<DateTime<Utc>>,
    current_end_time: Option<DateTime<Utc>>,
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
            .unwrap_or_else(|| Utc::now() - Duration::days(180));

        let end_time = &selection.candles_selection.end_time.unwrap_or_else(Utc::now);

        let candles = self
            .repo
            .candles_by_time(&selection.candles_selection.symbol_minutes, &start_time, &end_time);

        let candles_result = CandlesResult::new(candles.unwrap_or_default());

        let max_start_time = candles_result.start_date.unwrap_or(*start_time).max(*start_time);
        let min_end_time = candles_result.end_date.unwrap_or(*end_time).max(*end_time);

        if candles_result.is_empty() || &max_start_time != start_time || &min_end_time != end_time {
            // get from exchange
            // increment minutes from start
            // decrement minutes from end
            let candles_from_exch =
                self.repo
                    .candles_by_time(&selection.candles_selection.symbol_minutes, start_time, end_time);
        }

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
        Ok(candles_result.candles)
        //candles.ok_or(anyhow!("Not found"))
    }
}
