use std::collections::HashMap;

use crate::{
    candles_provider::candles_exchange::CandlesExchange,
    candles_provider::{
        candles_buffer::CandlesBuffer, candles_provider::CandlesProviderTrait, candles_repo::CandlesRepo,
        candles_resolver::CandlesResolver,
    },
    config::selection::Selection,
    config::{candles_selection::CandlesSelection, symbol_minutes::SymbolMinutes},
    exchange::Exchange,
    model::candle::Candle,
    repository::Repository,
    utils::{datetime_to_str, str_to_datetime},
};
use anyhow::Result;
use futures::future::Select;

pub fn candles_selection(exchange: Exchange, repo: Repository, selection: Selection) -> Result<Vec<Candle>> {
    let exchange = Exchange::new()?;
    let repo = Repository::new()?;

    let candles = repo
        .candles_by_time(
            &selection.candles_selection.symbol_minutes,
            &str_to_datetime(&selection.candles_selection.start_time),
            &str_to_datetime(&selection.candles_selection.end_time),
        )
        .unwrap_or_default();

    Ok(candles)
}

pub struct CandlesRetriever {
    exchange: &'static Exchange,
    repo: &'static Repository,
    candles_buffer: HashMap<SymbolMinutes, CandlesResolver>,
}

impl CandlesRetriever {
    pub fn new(exchange: &'static Exchange, repo: &'static Repository) -> Self {
        Self {
            exchange,
            repo,
            candles_buffer: HashMap::new(),
            // candles_repo: HashMap::new(),
            // candles_exchange: HashMap::new(),
        }
    }

    pub fn candles(&mut self, candles_selection: &CandlesSelection) -> Vec<Candle> {
        if !self.candles_buffer.contains_key(&candles_selection.symbol_minutes) {
            let cr_exchange =
                CandlesResolver::new(&candles_selection, Box::new(CandlesExchange::new(&self.exchange)), None);

            let cr_repo = CandlesResolver::new(
                &candles_selection,
                Box::new(CandlesRepo::new(&self.repo)),
                Some(Box::new(cr_exchange)),
            );
            let cr_buffer = CandlesResolver::new(
                &candles_selection,
                Box::new(CandlesBuffer::new()),
                Some(Box::new(cr_repo)),
            );

            self.candles_buffer
                .insert(candles_selection.symbol_minutes.clone(), cr_buffer);
        }

        todo!()
    }
}
