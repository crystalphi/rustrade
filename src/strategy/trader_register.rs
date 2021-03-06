use super::trend::{Operation, Trend};
use chrono::{DateTime, Utc};
use colored::Colorize;
use log::debug;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

pub static STATE_BOUGHT: &str = "bought";
pub static STATE_SOLD: &str = "sold";

pub struct Position {
    state: Trend,
    balance_coin: Decimal,
    balance_usd: Decimal,
}

impl Position {
    pub fn new_from_coin(balance_coin: Decimal) -> Self {
        Self {
            state: Trend::Bought,
            balance_coin,
            balance_usd: dec!(0),
        }
    }
    pub fn new_from_usd(balance_usd: Decimal) -> Self {
        Self {
            state: Trend::Sold,
            balance_coin: dec!(0),
            balance_usd,
        }
    }

    pub fn state(&self) -> &Trend {
        &self.state
    }
}

#[derive(Clone)]
pub struct Trade {
    pub operation: Operation,
    pub now: DateTime<Utc>,
    pub price: Decimal,
}

impl Trade {
    pub fn new(operation: Operation, now: DateTime<Utc>, price: Decimal) -> Self {
        Self { operation, now, price }
    }
}

pub struct TraderRegister {
    position: Position,
    trades: Vec<Trade>,
}

impl TraderRegister {
    pub fn new(postion: Position) -> Self {
        Self {
            position: postion,
            trades: Vec::new(),
        }
    }

    pub fn register(&mut self, trade: Trade) {
        match trade.operation {
            // I have USB and must buy coin
            Operation::Buy => {
                let quantity_usd = self.position.balance_usd;
                let quantity_coin = quantity_usd / trade.price;

                self.position.balance_coin += quantity_coin;
                self.position.balance_usd -= quantity_usd;
            }
            // I have USB and must buy coin
            Operation::Sell => {
                let quantity_coin = self.position.balance_coin;
                let quantity_usd = quantity_coin * trade.price;

                self.position.balance_coin -= quantity_coin;
                self.position.balance_usd += quantity_usd;
            }
        };

        self.position.balance_coin = self.position.balance_coin.round_dp_with_strategy(8, RoundingStrategy::RoundDown);
        self.position.balance_usd = self.position.balance_usd.round_dp_with_strategy(8, RoundingStrategy::RoundDown);

        self.position.state = trade.operation.to_trend();

        let message = match self.position.state {
            Trend::Bought => format!("{} Bought price {} Balance USD {}", trade.now, trade.price, self.position.balance_usd).green(),
            Trend::Sold => format!("{} Sold price {} Balance USD {}", trade.now, trade.price, self.position.balance_usd).red(),
        };
        debug!("{}", message);

        self.trades.push(trade);
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn trades(&self) -> Vec<Trade> {
        self.trades.clone()
    }
}
