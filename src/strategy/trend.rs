#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Bought,
    Sold,
}

impl Trend {
    pub fn to_operation(&self) -> Operation {
        match self {
            Trend::Bought => Operation::Buy,
            Trend::Sold => Operation::Sell,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Buy,
    Sell,
}

impl Operation {
    pub fn to_trend(&self) -> Trend {
        match self {
            Operation::Buy => Trend::Bought,
            Operation::Sell => Trend::Sold,
        }
    }
}
