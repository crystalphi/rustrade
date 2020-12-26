use rust_decimal::Decimal;

pub struct Position {
    id: u64,
    // position SOLD / BOUGHT
    state: String,
    // @Column(name = "BALANCE", precision = 5, scale = 2)
    // percent 
    balance: Decimal,
    // @Column(name = "BALANCE_USD", precision = 18, scale = 8)
    balance_usd: Decimal,
    // @Column(name = "BALANCE_CURRENCY", precision = 18, scale = 8)
    balance_amount: Decimal,
    // @Column(name = "BALANCE_MINIMUN", precision = 18, scale = 8)
    balance_minimun: Decimal,
    symbol: String,
    simulation: bool,
    active: bool,
    current_profit: u64,
}
