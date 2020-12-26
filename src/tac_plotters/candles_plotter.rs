use super::indicator_plotter::PlotterIndicatorContext;
use crate::{config::selection::Selection, model::candle::Candle};
use chrono::{DateTime, Utc};
use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_bitmap::bitmap_pixel::RGBPixel;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

pub struct CandlePlotter<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> CandlePlotter<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        CandlePlotter { candles }
    }
}

impl<'a> PlotterIndicatorContext for CandlePlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        chart_context.configure_mesh().x_labels(12).light_line_style(&WHITE).draw()?;

        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);
        // Into::<ShapeStyle>::into(&RGBColor(16, 196, 64)).filled(),
        // Into::<ShapeStyle>::into(&RGBColor(164, 16, 64)).filled(),

        let candle_series = self.candles.iter().map(|x| {
            CandleStick::new(
                x.close_time,
                x.open.to_f32().unwrap(),
                x.high.to_f32().unwrap(),
                x.low.to_f32().unwrap(),
                x.close.to_f32().unwrap(),
                &green,
                &red,
                4,
            )
        });
        chart_context.draw_series(candle_series)?;
        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        let max = self.candles.iter().fold(dec!(0), |acc, t| acc.max(t.high));
        let min = self.candles.iter().fold(max, |acc, t| acc.min(t.low));
        (min.to_f64().unwrap(), max.to_f64().unwrap())
    }
}
