use serde::{Deserialize, Serialize};

use super::definition::TacDefinition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Selection {
    pub tacs: Vec<TacDefinition>, //[{name: mcad, indicators: [ "signal", "diff" ] }]
    pub minutes: u32,             //15,
    pub symbol: String,           //"BTCUSDT",
    pub period_start: String,     //"2020-10",
    pub period_end: String,       //"2020-10" },
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
