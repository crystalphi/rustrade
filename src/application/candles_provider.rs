use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    candles_range::candles_to_ranges_missing,
    config::{candles_selection::CandlesSelection, symbol_minutes::SymbolMinutes},
    exchange::Exchange,
    model::{candle::Candle, open_close::OpenClose},
    repository::Repository,
    technicals::heikin_ashi,
};
use anyhow::anyhow;
use chrono::{Duration, Utc};
use ifmt::iformat;
use log::{info, warn};

pub trait CandlesProvider {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>>;

    fn clone_provider(&self) -> Box<dyn CandlesProvider>;
}

pub struct CandlesProviderBufferSingleton {
    exchange: Exchange,
    repository: Repository,
    buffer: HashMap<SymbolMinutes, Vec<Candle>>,
}

impl CandlesProviderBufferSingleton {
    pub fn new(repository: Repository, exchange: Exchange) -> Self {
        Self {
            exchange,
            repository,
            buffer: HashMap::new(),
        }
    }

    fn candles(&mut self, candles_selection: CandlesSelection) -> anyhow::Result<Vec<Candle>> {
        info!("Initializing import");

        fn candles_to_buf(heikin_ashi: bool, candles: &mut Vec<Candle>, buff: &mut Vec<Candle>) {
            if heikin_ashi {
                let candles = candles.iter().collect::<Vec<_>>();
                let candles_ref = candles.as_slice();
                let mut candles = heikin_ashi::heikin_ashi(candles_ref);
                buff.append(&mut candles);
            } else {
                buff.append(candles);
            }
            buff.sort();
        }

        // Normalize default start/end date time
        let start_time = &candles_selection.start_time.unwrap_or_else(|| Utc::now() - Duration::days(180));
        let end_time = &candles_selection.end_time.unwrap_or_else(Utc::now);
        let minutes = &candles_selection.symbol_minutes.minutes;
        let symbol_minutes = &candles_selection.symbol_minutes;

        // Get candles from buffer
        let mut candles_buf = self.buffer.entry(symbol_minutes.clone()).or_default();
        let ranges_missing = candles_to_ranges_missing(
            &OpenClose::from_date(start_time, minutes),
            &OpenClose::from_date(end_time, minutes),
            &candles_selection.symbol_minutes.minutes,
            candles_buf.iter().collect::<Vec<_>>().as_slice(),
        )?;

        for range_missing in ranges_missing.iter() {
            let start_time = range_missing.0;
            let end_time = range_missing.1;

            // Get candles from repository
            let mut candles_repo = self
                .repository
                .candles_by_time(&candles_selection.symbol_minutes, &start_time.open(minutes), &end_time.open(minutes))
                .unwrap_or_default();

            candles_to_buf(candles_selection.heikin_ashi, &mut candles_repo, &mut candles_buf);

            loop {
                // Get ranges missing
                let ranges_missing = candles_to_ranges_missing(
                    &start_time,
                    &end_time,
                    &candles_selection.symbol_minutes.minutes,
                    candles_repo.iter().collect::<Vec<_>>().as_slice(),
                )?;
                info!("Range {} {}. Missing candles: {}", start_time, end_time, ranges_missing.len());
                if ranges_missing.is_empty() {
                    break;
                }

                for range_missing in ranges_missing.iter() {
                    let msg = iformat!("Missing range: {range_missing:?}").to_string();
                    warn!("{}", msg);

                    let mut candles_exch = self.exchange.candles(
                        &candles_selection.symbol_minutes,
                        &Some(range_missing.0.open(minutes)),
                        &Some(range_missing.1.open(minutes)),
                    )?;

                    // Save news candles on repository
                    self.repository.add_candles(&mut candles_exch)?;

                    // Insert candles on buffer
                    candles_to_buf(candles_selection.heikin_ashi, &mut candles_exch, &mut candles_buf);
                }
            }
        }

        info!("Finished import");
        // candles_buf.iter().collect::<Vec<_>>()
        Ok(candles_buf.to_vec())
    }
}

#[derive(Clone)]
pub struct CandlesProviderBuffer {
    candles_provider_singleton: Rc<RefCell<CandlesProviderBufferSingleton>>,
    candles_selection_opt: Option<CandlesSelection>,
}

impl CandlesProviderBuffer {
    pub fn new(candles_provider_singleton: Rc<RefCell<CandlesProviderBufferSingleton>>) -> Self {
        Self {
            candles_provider_singleton,
            candles_selection_opt: None,
        }
    }
    pub fn set_candles_selection(&mut self, candles_selection: CandlesSelection) {
        self.candles_selection_opt = Some(candles_selection);
    }
}

impl CandlesProvider for CandlesProviderBuffer {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        let candles_selection = self
            .candles_selection_opt
            .as_ref()
            .cloned()
            .ok_or_else(|| -> anyhow::Error { anyhow!("candles_selection not definied!") })?;
        self.candles_provider_singleton.borrow_mut().candles(candles_selection)
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        todo!()
    }
}

pub struct CandlesProviderSelection {
    candles_provider: CandlesProviderBuffer,
    candles_selection: CandlesSelection,
}

impl<'a> CandlesProviderSelection {
    pub fn new(candles_provider: CandlesProviderBuffer, candles_selection: CandlesSelection) -> Self {
        Self {
            candles_provider,
            candles_selection,
        }
    }

    pub fn candles_selection(&self) -> CandlesSelection {
        self.candles_selection.clone()
    }
}

impl<'a> CandlesProvider for CandlesProviderSelection {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        self.candles_provider.set_candles_selection(self.candles_selection.clone());
        self.candles_provider.candles()
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(self.candles_provider.clone(), self.candles_selection.clone()))
    }
}

pub struct CandlesProviderVec {
    candles: Vec<Candle>,
}

impl CandlesProviderVec {
    pub fn new(candles: Vec<Candle>) -> Self {
        Self { candles }
    }
}

impl CandlesProvider for CandlesProviderVec {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        Ok(self.candles.to_vec())
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(self.candles.clone()))
    }
}

pub struct CandlesProviderClosure<'a, F>
where
    F: FnMut(CandlesSelection) -> anyhow::Result<Vec<&'a Candle>>,
{
    call_back: F,
}

impl<'a, F> CandlesProviderClosure<'a, F>
where
    F: FnMut(CandlesSelection) -> anyhow::Result<Vec<&'a Candle>>,
{
    pub fn new(call_back: F) -> Self
    where
        F: FnMut(CandlesSelection) -> anyhow::Result<Vec<&'a Candle>>,
    {
        Self { call_back }
    }

    pub fn candles(&mut self, candles_selection: CandlesSelection) -> anyhow::Result<Vec<&'a Candle>> {
        (self.call_back)(candles_selection)
    }
}
