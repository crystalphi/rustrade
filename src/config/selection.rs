use super::{candles_selection::CandlesSelection, definition::TacDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Selection {
    pub tacs: HashMap<String, TacDefinition>, //[{name: mcad, indicators: [ "signal", "diff" ] }]
    pub candles_selection: CandlesSelection,
    pub image_name: String,
}

impl Selection {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(text: &str) -> Self {
        serde_json::from_str(&text).unwrap()
    }
}
