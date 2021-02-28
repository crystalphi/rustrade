use super::indicator_plotter::PlotterIndicatorContext;
use crate::{
    config::selection::Selection,
    strategy::{trader_register::Trade, trend::Operation},
};
use chrono::{DateTime, Utc};
use plotters::{
    coord::types::RangedCoordf32,
    prelude::{Cartesian2d, ChartContext, Circle, EmptyElement, PointSeries, RangedDateTime},
    style::RGBColor,
    style::ShapeStyle,
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

pub struct TradingPlotter<'a> {
    trades: &'a [Trade],
}

impl<'a> TradingPlotter<'a> {
    pub fn new(trades: &'a [Trade]) -> Self {
        TradingPlotter { trades }
    }
}

impl<'a> PlotterIndicatorContext for TradingPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);

        let trades = &self.trades;

        let lows = PointSeries::of_element(
            trades
                .iter()
                .filter(|p| p.operation == Operation::Sell)
                .map(|c| (c.now, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&red).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size * 3, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );
        chart_context.draw_series(lows)?;

        let tops = PointSeries::of_element(
            trades
                .iter()
                .filter(|p| p.operation == Operation::Buy)
                .map(|c| (c.now, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&green).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size * 3, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );

        chart_context.draw_series(tops)?;
        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        let max = self.trades.iter().fold(dec!(0), |acc, t| acc.max(t.price));
        let min = self.trades.iter().fold(max, |acc, t| acc.min(t.price));
        (min.to_f64().unwrap(), max.to_f64().unwrap())
    }
}
