use crate::{
    application::candles_provider::{CandlesProvider, CandlesProviderBuffer, CandlesProviderSelection},
    config::{candles_selection::CandlesSelection, now_provider::MockNowProvider},
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType, indicator::Indicator},
};

pub struct TradeContextProvider {
    symbol: String,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    now_provider: MockNowProvider,
}

impl<'a> TradeContextProvider {
    pub fn new(symbol: &str, indicator_provider: IndicatorProvider, candles_provider: CandlesProviderBuffer) -> Self {
        Self {
            symbol: symbol.to_string(),
            indicator_provider,
            candles_provider,
            now_provider: MockNowProvider::new(),
        }
    }

    pub fn indicator(&mut self, minutes: u32, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let candles_selection = CandlesSelection::last_n(&self.symbol, &minutes, 200, &self.now_provider);

        // TODO PROVIDER MUST PASSING SELECTION
        let candles_provider_selection = CandlesProviderSelection::new(self.candles_provider.clone(), candles_selection);
        let candles_provider = Box::new(candles_provider_selection) as Box<dyn CandlesProvider>;

        self.indicator_provider.indicator(candles_provider, i_type)
    }
}
