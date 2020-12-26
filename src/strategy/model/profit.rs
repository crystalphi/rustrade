use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Profit {
    id: u64,
    symbol: String,
    position: u64,
    sold_date: DateTime<Utc>,
    // @Column(name = "SOLD_PRICE", precision = 14, scale = 8)
    sold_price: Decimal,
    // @Column(name = "SOLD_AMOUNT", precision = 14, scale = 8)
    sold_amount: Decimal,
    bought_date: DateTime<Utc>,
    // @Column(name = "BOUGHT_PRICE", precision = 14, scale = 8)
    bought_price: Decimal,
    // @Column(name = "BOUGHT_AMOUNT", precision = 14, scale = 8)
    bought_amount: Decimal,
    // @Column(name = "BOUGHT_USD", precision = 14, scale = 8)
    bought_usd: Decimal,
    // @Column(name = "SOLD_USD", precision = 14, scale = 8)
    sold_usd: Decimal,
    // @Column(name = "BOUGHT_ORDER")
    bought_order: u64,
    // @Column(name = "SOLD_ORDER")
    sold_order: u64,
    // @Column(name = "PROFIT", precision = 14, scale = 8)
    profit: Decimal,
    // @Column(name = "SIMULATED")
    simulation: bool,
    // @Column(name = "PERCENT", precision = 5, scale = 2)
    percent: Decimal,
}
