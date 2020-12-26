#[derive(PartialEq, Eq, Hash)]
pub enum IndicatorType {
    Macd(usize, usize, usize),
    Macd_signal(usize, usize, usize),
    Macd_divergence(usize, usize, usize),
    Ema(usize),
    Sma(usize),
    Pivot(usize),
}
