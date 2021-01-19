use super::trend::{Operation, Trend};
use chrono::{DateTime, Utc};
use colored::Colorize;
use log::info;
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

pub struct Trade {
    operation: Operation,
    price: Decimal,
}

impl Trade {
    pub fn new(operation: Operation, price: Decimal) -> Self {
        Self { operation, price }
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

    pub fn register(&mut self, now: DateTime<Utc>, operation: Operation, price: Decimal) {
        match operation {
            // I have USB and must buy coin
            Operation::Buy => {
                let quantity_usd = self.position.balance_usd;
                let quantity_coin = quantity_usd / price;

                self.position.balance_coin += quantity_coin;
                self.position.balance_usd -= quantity_usd;
            }
            // I have USB and must buy coin
            Operation::Sell => {
                let quantity_coin = self.position.balance_coin;
                let quantity_usd = quantity_coin * price;

                self.position.balance_coin -= quantity_coin;
                self.position.balance_usd += quantity_usd;
            }
        };

        self.position.balance_coin = self.position.balance_coin.round_dp_with_strategy(8, RoundingStrategy::RoundDown);
        self.position.balance_usd = self.position.balance_usd.round_dp_with_strategy(8, RoundingStrategy::RoundDown);

        self.position.state = operation.to_trend();

        let message = match self.position.state {
            Trend::Bought => format!("{} Bought {} Balance USD {}", now, price, self.position.balance_usd).green(),
            Trend::Sold => format!("{} Sold {} Balance USD {}", now, price, self.position.balance_usd).red(),
        };
        info!("{}", message);

        self.trades.push(Trade::new(operation, price));
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn trades(&self) -> &Vec<Trade> {
        &self.trades
    }
}
