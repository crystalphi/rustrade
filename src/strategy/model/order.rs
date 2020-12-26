use rust_decimal::Decimal;

pub struct Order {
    //@Column(name = "ID")
    id: u64,
    //@Column(name = "SYMBOL")
    symbol: String,
    //@Column(name = "EXCHANGE")
    exchange: String,
    //@Column(name = "PRICE", precision = 14, scale = 8)
    price: Decimal,
    //@Column(name = "AVG_EXECUTION_PRICE", precision = 14, scale = 8)
    avg_execution_price: Decimal,
    // buy / sell
    //@Column(name = "SIDE")
    side: String,
    // exchange limit
    //@Column(name = "TYPE")
    type_o: String,
    //@Column(name = "IS_LIVE")
    is_live: bool,
    //@Column(name = "IS_CANCELLED")
    is_cancelled: bool,
    //@Column(name = "IS_HIDDEN")
    is_hidden: bool,
    //@Column(name = "WAS_FORCED")
    was_forced: bool,
    //@Column(name = "ORIGINAL_AMOUNT", precision = 18, scale = 8)
    original_amount: Decimal,
    //@Column(name = "REMAINING_AMOUNT", precision = 18, scale = 8)
    remaining_amount: Decimal,
    //@Column(name = "EXECUTED_AMOUNT", precision = 18, scale = 8)
    executed_amount: Decimal,
    //@Column(name = "SOURCE")
    source: String,
    //@Column(name = "ORDER_ID")
    order_id: u64,
    //@Column(name = "POSITION")
    position: u64,
    //@Column(name = "PROFIT")
    profit: u64,
    //@Column(name = "SIMULATED")
    simulation: bool,
}
