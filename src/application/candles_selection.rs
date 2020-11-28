use crate::{
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
