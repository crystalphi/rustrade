use chrono::{DateTime, Utc};
use plotters::{
    coord::types::RangedCoordf32,
    prelude::{Cartesian2d, ChartContext, Circle, EmptyElement, PointSeries, RangedDateTime},
    style::RGBColor,
    style::ShapeStyle,
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};
use rust_decimal::prelude::ToPrimitive;

use crate::technicals::pivots::{Pivot, PivotType};

use super::indicator_plotter::PlotterIndicatorContext;

pub struct PivotPlotter<'a> {
    pivots: &'a [Pivot<'a>],
}

impl<'a> PivotPlotter<'a> {
    pub fn new(pivots: &'a [Pivot<'a>]) -> Self {
        PivotPlotter { pivots }
    }
}

impl<'a> PlotterIndicatorContext for PivotPlotter<'a> {
    fn plot(
        &self,
        _from_date: &DateTime<Utc>,
        _to_date: &DateTime<Utc>,

        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);

        let pivots = self.pivots;

        // let pivots = self
        //     .pivots
        //     .iter()
        //     .filter(|p| p.close_time >= from_date && p.close_time <= to_date)
        //     .collect::<Vec<_>>();

        let low_pivots = PointSeries::of_element(
            pivots
                .iter()
                .filter(|p| p.type_p == PivotType::Low)
                .map(|c| (*c.close_time, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&red).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );
        chart_context.draw_series(low_pivots)?;

        let high_pivots = PointSeries::of_element(
            pivots
                .iter()
                .filter(|p| p.type_p == PivotType::High)
                .map(|c| (*c.close_time, c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&green).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );

        chart_context.draw_series(high_pivots)?;
        Ok(())
    }
}
