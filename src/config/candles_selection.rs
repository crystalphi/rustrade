use super::symbol_minutes::SymbolMinutes;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct CandlesSelection {
    pub symbol_minutes: SymbolMinutes,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

impl CandlesSelection {
    pub fn new(symbol: &str, minutes: &u32, start_time: Option<&str>, end_time: Option<&str>) -> Self {
        Self {
            symbol_minutes: SymbolMinutes::new(symbol, minutes),
            start_time: start_time.map(|s| s.to_owned()),
            end_time: end_time.map(|s| s.to_owned()),
        }
    }
}
