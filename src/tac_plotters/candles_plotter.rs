use chrono::{DateTime, Utc};

use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_bitmap::bitmap_pixel::RGBPixel;
use rust_decimal::prelude::ToPrimitive;

use crate::model::candle::Candle;

use super::indicator_plotter::PlotterIndicatorContext;

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
        from_date: &DateTime<Utc>,
        to_date: &DateTime<Utc>,

        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);

        chart_context.configure_mesh().x_labels(12).light_line_style(&WHITE).draw()?;

        let candle_series = self.candles.iter().map(|x| {
            CandleStick::new(
                x.close_time,
                x.open.to_f32().unwrap(),
                x.high.to_f32().unwrap(),
                x.low.to_f32().unwrap(),
                x.close.to_f32().unwrap(),
                &green,
                &red,
                2,
            )
        });
        chart_context.draw_series(candle_series)?;
        Ok(())
    }
}
