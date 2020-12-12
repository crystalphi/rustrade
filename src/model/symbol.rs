use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    BTCUSDT,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<String> for Symbol {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value[..] {
            "BTCUSDT" => Ok(Symbol::BTCUSDT),
            _ => Err(format!("Content {} is not valid symbol!", value)),
        }
    }
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "BTCUSDT" => Ok(Symbol::BTCUSDT),
            _ => Err(format!("Content {} is not valid symbol!", value)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn symbol_test() {
        println!("{}", Symbol::BTCUSDT);
    }
}
