use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct SymbolMinutes {
    pub symbol: String,
    pub minutes: u32,
}

impl SymbolMinutes {
    pub fn new(symbol: &str, minutes: &u32) -> Self {
        Self {
            symbol: symbol.into(),
            minutes: *minutes,
        }
    }
}
