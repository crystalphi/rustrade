use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TacDefinition {
    pub name: String,
    pub indicators: HashSet<String>,
}

impl TacDefinition {
    pub fn new(name: &str, indicators: &[&str]) -> Self {
        TacDefinition {
            name: name.into(),
            indicators: indicators.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDefinition {
    tacs: Vec<TacDefinition>, //[{name: mcad, indicators: [ "signal", "diff" ] }]
    minutes: Vec<u32>,        //[1,5,15],
    symbol: Vec<String>,      //["BTCUSDT", "IOTUSDT"],
    period_start: String,     //"2020-10-01",//"2020-10",
    period_end: String,       // "2020-11-01",
}

impl ConfigDefinition {
    pub fn new() -> Self {
        ConfigDefinition {
            tacs: vec![TacDefinition::new("pivots", &["pivots"])],
            minutes: vec![5u32, 15u32, 30u32, 60u32],
            symbol: vec!["BTCUSDT".to_string()],
            period_start: "2020-06-01 00:00:00".to_string(),
            period_end: "2020-11-30 00:00:00".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for ConfigDefinition {
    fn default() -> Self {
        Self::new()
    }
}
