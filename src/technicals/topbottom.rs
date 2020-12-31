use super::technical::{TechnicalDefinition, TechnicalIndicators};
use crate::{config::definition::TacDefinition, model::candle::Candle};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::{cmp::Ordering, collections::HashSet};
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum TopBottomType {
    Top,
    Bottom,
}

impl std::fmt::Display for TopBottomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if *self == TopBottomType::Top { "Low" } else { "High" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopBottom<'a> {
    pub close_time: &'a DateTime<Utc>,
    pub price: &'a Decimal,
    pub type_p: TopBottomType,
}

impl<'a> TopBottom<'a> {
    pub fn new(type_p: TopBottomType, close_time: &'a DateTime<Utc>, price: &'a Decimal) -> Self {
        Self { close_time, type_p, price }
    }
}

impl<'a> PartialOrd for TopBottom<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.close_time.cmp(other.close_time))
    }
}

impl<'a> Ord for TopBottom<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.close_time.cmp(&other.close_time)
    }
}

pub struct TopBottomTac<'a> {
    candles: &'a [&'a Candle],
    neighbors: usize,
}

impl<'a> TechnicalDefinition<'a> for TopBottomTac<'a> {
    fn definition() -> TacDefinition {
        TacDefinition::new("topbottom", &["topbottom"])
    }
}

impl<'a> TechnicalIndicators<'a> for TopBottomTac<'a> {
    fn indicators(&self) -> &std::collections::HashMap<String, super::indicator::Indicator<'a>> {
        todo!()
    }

    fn main_indicator(&self) -> &super::indicator::Indicator {
        todo!()
    }
}

impl<'a> TopBottomTac<'a> {
    pub fn new(candles: &'a [&'a Candle], neighbors: usize) -> Self {
        TopBottomTac { candles, neighbors }
    }

    pub fn topbottoms(&self) -> Vec<TopBottom<'a>> {
        let mut result = Vec::new();
        for i in 0..self.candles.len() - (self.neighbors * 2 + 1) {
            let candle = self.candles[i + self.neighbors];
            let l_min = self.candles[i..i + self.neighbors].iter().map(|c| c.low).min().unwrap_or(candle.low);
            let l_max = self.candles[i..i + self.neighbors].iter().map(|c| c.high).max().unwrap_or(candle.high);
            let r_min = self.candles[i + self.neighbors + 1..i + (self.neighbors * 2 + 1)]
                .iter()
                .map(|c| c.low)
                .min()
                .unwrap_or(candle.low);
            let r_max = self.candles[i + self.neighbors + 1..i + (self.neighbors * 2 + 1)]
                .iter()
                .map(|c| c.high)
                .max()
                .unwrap_or(candle.high);
            if candle.low < l_min && candle.low < r_min {
                result.push(TopBottom::new(TopBottomType::Top, &candle.close_time, &candle.low));
            }
            if candle.high > l_max && candle.high > r_max {
                result.push(TopBottom::new(TopBottomType::Bottom, &candle.close_time, &candle.high));
            }
        }
        normalize_topbottoms(&mut result);
        result
    }
}

fn normalize_topbottoms(topbottoms: &mut Vec<TopBottom>) {
    if topbottoms.is_empty() {
        return;
    }

    let mut delete = HashSet::new();
    let mut reverse = topbottoms.clone();
    reverse.reverse();

    let mut topbottoms_iter = reverse.iter();

    let mut previous = topbottoms_iter.next().unwrap();
    loop {
        match topbottoms_iter.next() {
            None => break,
            Some(current) => {
                if current.type_p == previous.type_p {
                    if current.type_p == TopBottomType::Top {
                        delete.insert(max_price(previous, current));
                    } else {
                        delete.insert(min_price(previous, current));
                    }
                }
                previous = current;
            }
        }
    }

    topbottoms.retain(|p| delete.get(p).is_none());
}

fn max_price<'a>(previous: &'a TopBottom, current: &'a TopBottom) -> &'a TopBottom<'a> {
    if previous.price > current.price {
        previous
    } else {
        current
    }
}

fn min_price<'a>(previous: &'a TopBottom, current: &'a TopBottom) -> &'a TopBottom<'a> {
    if previous.price < current.price {
        previous
    } else {
        current
    }
}

#[cfg(test)]
pub mod tests {
    use crate::utils::str_to_datetime;

    use super::*;
    use ifmt::iprintln;
    use rust_decimal_macros::dec;

    #[test]
    fn topbottom_test() {
        let c1 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 12:00:00"),
            close_time: str_to_datetime("2020-01-12 12:14:59"),
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
            open_time: str_to_datetime("2020-01-12 12:15:00"),
            close_time: str_to_datetime("2020-01-12 12:29:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(102.0),
            low: dec!(102.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c3 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 12:30:00"),
            close_time: str_to_datetime("2020-01-12 12:44:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(103.0),
            low: dec!(103.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c4 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 12:45:00"),
            close_time: str_to_datetime("2020-01-12 12:59:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(104.0),
            low: dec!(104.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c5 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 13:00:00"),
            close_time: str_to_datetime("2020-01-12 13:14:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(105.0),
            low: dec!(105.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c6 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 13:15:00"),
            close_time: str_to_datetime("2020-01-12 13:29:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(106.0),
            low: dec!(106.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c7 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 13:30:00"),
            close_time: str_to_datetime("2020-01-12 13:44:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(107.0),
            low: dec!(107.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c8 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 13:45:00"),
            close_time: str_to_datetime("2020-01-12 13:59:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(108.0),
            low: dec!(108.0),
            close: dec!(108.0),
            volume: dec!(100.0),
        };

        let c9 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 14:00:00"),
            close_time: str_to_datetime("2020-01-12 14:14:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(107.0),
            low: dec!(107.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c10 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 14:15:00"),
            close_time: str_to_datetime("2020-01-12 14:29:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(106.0),
            low: dec!(106.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c11 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 14:30:00"),
            close_time: str_to_datetime("2020-01-12 14:44:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(105.0),
            low: dec!(105.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c12 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 14:45:00"),
            close_time: str_to_datetime("2020-01-12 14:59:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(104.0),
            low: dec!(104.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c13 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 15:00:00"),
            close_time: str_to_datetime("2020-01-12 15:14:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(103.0),
            low: dec!(103.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c14 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 15:15:00"),
            close_time: str_to_datetime("2020-01-12 15:29:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(102.0),
            low: dec!(102.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c15 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 15:30:00"),
            close_time: str_to_datetime("2020-01-12 15:44:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(101.0),
            low: dec!(101.0),
            close: dec!(101.0),
            volume: dec!(100.0),
        };

        let c16 = Candle {
            id: dec!(0),
            open_time: str_to_datetime("2020-01-12 15:45:00"),
            close_time: str_to_datetime("2020-01-12 15:59:59"),
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
            open_time: str_to_datetime("2020-01-12 16:00:00"),
            close_time: str_to_datetime("2020-01-12 16:14:59"),
            symbol: "BTCUSDT".into(),
            minutes: dec!(15),
            open: dec!(100.0),
            high: dec!(99.0),
            low: dec!(99.0),
            close: dec!(99.0),
            volume: dec!(100.0),
        };

        let candles = [&c1, &c2, &c3, &c4, &c5, &c6, &c7, &c8, &c9, &c10, &c11, &c12, &c13, &c14, &c15, &c16, &c17];

        let topbottom_tac = TopBottomTac::new(&candles, 7);

        let topbottoms = topbottom_tac.topbottoms();

        iprintln!("{topbottoms.len()}");
        for topbottom in topbottoms.iter() {
            iprintln!("{topbottom:?}");
        }
    }
}
