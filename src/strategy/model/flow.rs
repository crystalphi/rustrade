use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Flow {
    // @Column(name = "PROFIT")
    profit: u64,
    // @Column(name = "POSITION")
    position: u64,
    // @Column(name = "DATE")
    // @Temporal(TemporalType.TIMESTAMP)
    date: DateTime<Utc>,
    // @Column(name = "PRICE", precision = 14, scale = 8)
    price: Decimal,
    // @Column(name = "AMOUNT", precision = 14, scale = 8)
    amount: Decimal,
    // @Column(name = "USD", precision = 14, scale = 8)
    usd: Decimal,
    // @Column(name = "ORIGINAL_USD", precision = 14, scale = 8)
    original_usd: Decimal,
    // @Column(name = "BALANCE_AMOUNT", precision = 14, scale = 8)
    balance_amount: Decimal,
    // @Column(name = "BALANCE_USD", precision = 14, scale = 8)
    balance_usd: Decimal,
    // @Column(name = "STATE")
    state: String,
}
