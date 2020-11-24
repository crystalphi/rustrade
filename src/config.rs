use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TacDefinition {
    name: String,
    indicators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDefinition {
    tacs: Vec<TacDefinition>, //[{name: mcad, indicators: [ "signal", "diff" ] }]
    minutes: Vec<u32>,        //[1,5,15],
    symbol: Vec<String>,      //["BTCUSDT", "IOTUSDT"],
    period_start: String,     //"2020-10-01",//"2020-10",
    period_end: String,       // "2020-11-01",
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Selection {
    tacs: Vec<TacDefinition>, //[{name: mcad, indicators: [ "signal", "diff" ] }]
    minutes: u32,             //15,
    symbol: String,           //"BTCUSDT",
    period_start: String,     //"2020-10",
    period_end: String,       //"2020-10" },
}

pub struct Configuration {}

impl Configuration {
    pub fn init() {}
}

// https://serde.rs/derive.html

// let serialized = serde_json::to_string(&point).unwrap();
// println!("serialized = {}", serialized);

// let deserialized: Point = serde_json::from_str(&serialized).unwrap();
// println!("deserialized = {:?}", deserialized);

pub struct Command {}
