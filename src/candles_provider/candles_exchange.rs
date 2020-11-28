use crate::{exchange::Exchange, model::candle::Candle, utils::str_to_datetime};

use super::candles_provider::CandlesProviderTrait;

pub struct CandlesExchange<'a> {
    candle_provider: Option<&'a dyn CandlesProviderTrait>,
    exchange: &'a Exchange,
}

impl<'a> CandlesExchange<'a> {
    pub fn new(candle_provider: Option<&'a dyn CandlesProviderTrait>, exchange: &'a Exchange) -> Self {
        CandlesExchange {
            candle_provider,
            exchange,
        }
    }
}

impl<'a> CandlesProviderTrait for CandlesExchange<'a> {
    fn candles(&self, symbol: &str, minutes: &u32, start_time: &str, end_time: &str) -> Vec<Candle> {
        self.exchange.candles(
            symbol,
            minutes,
            &Some(str_to_datetime(start_time)),
            &Some(str_to_datetime(end_time)),
        )
    }
}
