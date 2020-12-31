pub struct Configuration {
    selection: Selection,
}

impl Configuration {
    pub fn init() -> Configuration {}
}

// https://serde.rs/derive.html

// let serialized = serde_json::to_string(&point).unwrap();
// println!("serialized = {}", serialized);

// let deserialized: Point = serde_json::from_str(&serialized).unwrap();
// println!("deserialized = {:?}", deserialized);

pub struct Command {}
