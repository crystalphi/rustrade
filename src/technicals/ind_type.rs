#[derive(PartialEq, Eq, Hash)]
pub enum IndicatorType {
    Macd(usize, usize, usize),
    MacdSignal(usize, usize, usize),
    MacdDivergence(usize, usize, usize),
    Ema(usize),
    Sma(usize),
    TopBottom(usize),
}
