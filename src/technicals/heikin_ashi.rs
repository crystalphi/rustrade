use crate::model::candle::Candle;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const DEC_2: Decimal = dec!(2);
const DEC_4: Decimal = dec!(4);

/// Heikin-Ashi Candle Calculations
pub fn heikin_ashi(candles: &[&Candle]) -> Vec<Candle> {
    let mut ha_candles = Vec::with_capacity(candles.len());
    fn add_ha(has: &mut Vec<Candle>, hac: Candle) -> (Decimal, Decimal) {
        let prev_oc = (hac.open, hac.close);
        has.push(hac);
        prev_oc
    }

    let first_ha = heikin_ashi_first(&candles[0]);
    let mut prev_oc = add_ha(&mut ha_candles, first_ha);

    for candle in &candles[1..] {
        let ha = heikin_ashi_candles(prev_oc.0, prev_oc.1, &candle);
        prev_oc = add_ha(&mut ha_candles, ha);
    }
    ha_candles
}

/// First Heikin-Ashi Candle Calculations
fn heikin_ashi_first(c: &Candle) -> Candle {
    Candle {
        close: (c.open + c.high + c.low + c.close) / DEC_4,
        open: (c.open + c.close) / DEC_2,
        ..c.clone()
    }
}

/// Heikin-Ashi Candle Calculations
fn heikin_ashi_candles(prev_ha_open: Decimal, prev_ha_close: Decimal, c: &Candle) -> Candle {
    let ha_close = (c.open + c.high + c.low + c.close) / DEC_4;
    let ha_open = (prev_ha_open + prev_ha_close) / DEC_2;
    let ha_low = c.low.min(ha_open).min(ha_close);
    let ha_high = c.high.max(ha_open).max(ha_close);
    Candle {
        close: ha_close,
        open: ha_open,
        low: ha_low,
        high: ha_high,
        ..c.clone()
    }
}
