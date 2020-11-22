use crate::model::candle::Candle;

pub struct PivotTac<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> PivotTac<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        PivotTac { candles }
    }

    pub fn pivots(&self) -> (Vec<&'a Candle>, Vec<&'a Candle>) {
        //     0..7    8..15
        // 012345678901234
        //        X0123456

        let mut result_min = Vec::new();
        let mut result_max = Vec::new();

        for i in 0..self.candles.len() - 15 {
            let pivot = self.candles[i + 7];

            let l_min = self.candles[i..i + 7]
                .iter()
                .map(|c| c.close)
                .min()
                .unwrap_or(pivot.close);

            let l_max = self.candles[i..i + 7]
                .iter()
                .map(|c| c.close)
                .max()
                .unwrap_or(pivot.close);

            let r_min = self.candles[i + 8..i + 15]
                .iter()
                .map(|c| c.close)
                .min()
                .unwrap_or(pivot.close);

            let r_max = self.candles[i + 8..i + 15]
                .iter()
                .map(|c| c.close)
                .max()
                .unwrap_or(pivot.close);

            if pivot.close < l_min && pivot.close < r_min {
                result_min.push(pivot);
            }

            if pivot.close > l_max && pivot.close > r_max {
                result_max.push(pivot);
            }
        }
        (result_min, result_max)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ifmt::iprintln;
    use rust_decimal_macros::dec;

    #[test]
    fn pivot_test() {
        let c1 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 12:00:00".into(),
            close_time: "2020-01-12 12:14:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(100.0),
            volume: dec!(100.0),
        };

        let c2 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 12:15:00".into(),
            close_time: "2020-01-12 12:29:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c3 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 12:30:00".into(),
            close_time: "2020-01-12 12:44:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c4 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 12:45:00".into(),
            close_time: "2020-01-12 12:59:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c5 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 13:00:00".into(),
            close_time: "2020-01-12 13:14:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c6 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 13:15:00".into(),
            close_time: "2020-01-12 13:29:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c7 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 13:30:00".into(),
            close_time: "2020-01-12 13:44:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c8 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 13:45:00".into(),
            close_time: "2020-01-12 13:59:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(108.0),
            volume: dec!(100.0),
        };

        let c9 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 14:00:00".into(),
            close_time: "2020-01-12 14:14:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c10 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 14:15:00".into(),
            close_time: "2020-01-12 14:29:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c11 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 14:30:00".into(),
            close_time: "2020-01-12 14:44:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c12 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 14:45:00".into(),
            close_time: "2020-01-12 14:59:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c13 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 15:00:00".into(),
            close_time: "2020-01-12 15:14:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c14 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 15:15:00".into(),
            close_time: "2020-01-12 15:29:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c15 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 15:30:00".into(),
            close_time: "2020-01-12 15:44:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(101.0),
            volume: dec!(100.0),
        };

        let c16 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 15:45:00".into(),
            close_time: "2020-01-12 15:59:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(100.0),
            volume: dec!(100.0),
        };

        let c17 = Candle {
            id: dec!(0),
            open_time: "2020-01-12 16:00:00".into(),
            close_time: "2020-01-12 16:14:59".into(),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(99.0),
            volume: dec!(100.0),
        };

        let candles = [
            &c1, &c2, &c3, &c4, &c5, &c6, &c7, &c8, &c9, &c10, &c11, &c12, &c13, &c14, &c15, &c16,
            &c17,
        ];

        let pivot_tac = PivotTac::new(&candles);

        let (pivots_min, pivots_max) = pivot_tac.pivots();

        iprintln!("{pivots_min.len()}");
        for pivot in pivots_min.iter() {
            iprintln!("{pivot}");
        }

        iprintln!("{pivots_max.len()}");
        for pivot in pivots_max.iter() {
            iprintln!("{pivot}");
        }
    }
}
