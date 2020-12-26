pub enum IndicatorType {
    Macd(u32, u32, u32),
    Ema(u32),
    Sma(u32),
    Pivot(u32),
}
