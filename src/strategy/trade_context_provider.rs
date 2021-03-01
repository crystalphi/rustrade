use crate::{
    application::candles_provider::{CandlesProvider, CandlesProviderBuffer, CandlesProviderSelection, CandlesProviderVec},
    config::candles_selection::CandlesSelection,
    model::candle::Candle,
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType, indicator::Indicator},
};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct TradeContextProvider {
    symbol: String,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    now: DateTime<Utc>,
    candles_opt: Option<(DateTime<Utc>, u32, Vec<Candle>)>,
}

impl TradeContextProvider {
    pub fn new(symbol: &str, indicator_provider: IndicatorProvider, candles_provider: CandlesProviderBuffer) -> Self {
        Self {
            symbol: symbol.to_string(),
            indicator_provider,
            candles_provider,
            now: Utc::now(),
            candles_opt: None,
        }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = now;
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.now
    }

    pub fn indicator(&mut self, minutes: u32, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let now = self.now;
        self.candles_opt = self.candles_opt.take().filter(|e| e.0 == now && e.1 == minutes);

        let candles_selection = CandlesSelection::last_n(&self.symbol, &minutes, 200, now);
        let mut candles_provider_selection = CandlesProviderSelection::new(self.candles_provider.clone(), candles_selection);

        let now_candles = self.candles_opt.get_or_insert_with(|| {
            let candles = candles_provider_selection.candles().unwrap();
            (now, minutes, candles)
        });

        let candles_provider_vec = CandlesProviderVec::new(now_candles.2.as_slice(), 200);
        let candles_provider = Box::new(candles_provider_vec) as Box<dyn CandlesProvider>;

        self.indicator_provider.indicator(self.now, candles_provider, i_type)
    }
}
