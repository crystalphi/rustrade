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

    for candle in candles {
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

// https://quantiacs.com/Blog/Intro-to-Algorithmic-Trading-with-Heikin-Ashi.aspx

/*

Often times Heikin-Ashi is a purely visual aid, and as you can see from the charts it smooths out the bars to emphasize trends with streaks of increasing/decreasing bars.
To build our system, we’ll have to dive into the math behind the indicator.
For any indicator this is usually well documented and can be found with a quick Google search. Here’s what we get for Heikin-Ashi:

Heikin-Ashi Candle Calculations
HA_Close = (Open + High + Low + Close) / 4
HA_Open = (previous HA_Open + previous HA_Close) / 2
HA_Low = minimum of Low, HA_Open, and HA_Close
HA_High = maximum of High, HA_Open, and HA_Close

One thing you may notice immediately is that the Heikin-Ashi Open price is a result of the previous Heikin-Ashi values.
So when you’re first starting to calculate Heikin-Ashi, how do you obtain “previous” values? Well the standard solution is to do this on the first run:
Heikin-Ashi Calculations on First Run

HA_Close = (Open + High + Low + Close) / 4
HA_Open = (Open + Close) / 2
HA_Low = Low
HA_High = High


*/

// One thing you may notice immediately is that the Heikin-Ashi Open price is a result of the previous Heikin-Ashi values. So when you’re first starting to calculate Heikin-Ashi, how do you obtain “previous” values? Well the standard solution is to do this on the first run:
// Heikin-Ashi Calculations on First Run
