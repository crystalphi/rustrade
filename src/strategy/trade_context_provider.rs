use crate::{
    application::candles_provider::CandlesProvider,
    config::{
        candles_selection::CandlesSelection,
        now_provider::{MockNowProvider, NowProvider},
    },
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType, indicator::Indicator},
};

pub struct TradeContextProvider<'a> {
    symbol: &'a str,
    indicator_provider: IndicatorProvider<'a>,
    candles_provider: CandlesProvider<'a>,
    pub now_provider: MockNowProvider,
}

impl<'a> TradeContextProvider<'a> {
    pub fn new(symbol: &'a str, indicator_provider: IndicatorProvider<'a>, candles_provider: CandlesProvider<'a>) -> Self {
        Self {
            symbol,
            indicator_provider,
            candles_provider,
            now_provider: MockNowProvider::new(),
        }
    }

    pub fn indicator(&mut self, minutes: u32, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let candles_selection = CandlesSelection::last_n(self.symbol, &minutes, 200, &self.now_provider);
        let candles_own = self.candles_provider.candles_selection(&candles_selection)?;
        let candles = candles_own.iter().collect::<Vec<_>>();
        let candles_ref = candles.as_ref();
        self.indicator_provider.indicator(candles_ref, i_type)
    }

    pub fn last_candle(&self) {}
}
