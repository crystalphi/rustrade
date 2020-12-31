use super::indicator_plotter::PlotterIndicatorContext;
use crate::{
    config::selection::Selection,
    technicals::topbottom::{TopBottom, TopBottomType},
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

pub struct TopBottomPlotter<'a> {
    topbottoms: &'a [TopBottom<'a>],
}

impl<'a> TopBottomPlotter<'a> {
    pub fn new(topbottoms: &'a [TopBottom<'a>]) -> Self {
        TopBottomPlotter { topbottoms }
    }
}

impl<'a> PlotterIndicatorContext for TopBottomPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);

        let topbottoms = self.topbottoms;

        let lows = PointSeries::of_element(
            topbottoms
                .iter()
                .filter(|p| p.type_p == TopBottomType::Top)
                .map(|c| (*c.close_time, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&red).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );
        chart_context.draw_series(lows)?;

        let tops = PointSeries::of_element(
            topbottoms
                .iter()
                .filter(|p| p.type_p == TopBottomType::Bottom)
                .map(|c| (*c.close_time, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&green).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );

        chart_context.draw_series(tops)?;
        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        let max = self.topbottoms.iter().fold(dec!(0), |acc, t| acc.max(*t.price));
        let min = self.topbottoms.iter().fold(max, |acc, t| acc.min(*t.price));
        (min.to_f64().unwrap(), max.to_f64().unwrap())
    }
}
