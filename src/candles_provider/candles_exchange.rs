use crate::{config::symbol_minutes::SymbolMinutes, exchange::Exchange, model::candle::Candle, utils::str_to_datetime};

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesExchange<'a> {
    exchange: &'a Exchange,
}

impl<'a> CandlesExchange<'a> {
    pub fn new(exchange: &'a Exchange) -> Self {
        CandlesExchange { exchange }
    }
}

impl<'a> CandlesProviderTrait for CandlesExchange<'a> {
    fn candles(&self, symbol_minutes: &SymbolMinutes, start_time: &str, end_time: &str) -> Vec<Candle> {
        self.exchange.candles(
            symbol_minutes,
            &Some(str_to_datetime(start_time)),
            &Some(str_to_datetime(end_time)),
        )
    }
}
