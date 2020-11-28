use crate::{
    candles_provider::candles_resolver::CandlesResolver,
    config::selection::Selection,
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
            &selection.symbol,
            &selection.minutes,
            &str_to_datetime(&selection.period_start),
            &str_to_datetime(&selection.period_end),
        )
        .unwrap_or_default();

    Ok(candles)
}

pub struct CandlesSelection<'a> {
    exchange: &'a Exchange,
    repo: &'a Repository,
    candles_buffer: CandlesResolver,
    candles_repo: CandlesResolver,
    candles_exchange: CandlesResolver,
}

impl<'a> CandlesSelection<'a> {
    pub fn new(exchange: &'a Exchange, repo: &'a Repository) -> Self {

        let candles_buffer = CandlesResolver::new(),
        let candles_repo: CandlesResolver,
        let candles_exchange: CandlesResolver,
    
        CandlesSelection { exchange, repo }
    }
}
