use super::indicator_plotter::PlotterIndicatorContext;
use crate::{config::selection::Selection, technicals::indicator::Indicator};
use chrono::{DateTime, Utc};
use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_bitmap::bitmap_pixel::RGBPixel;

pub struct LineIndicatorPlotter<'a> {
    indicator: &'a Indicator,
    color: RGBColor,
}

impl<'a> LineIndicatorPlotter<'a> {
    pub fn new(indicator: &'a Indicator, color: RGBColor) -> Self {
        Self { indicator, color }
    }
}

impl<'a> PlotterIndicatorContext for LineIndicatorPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        //chart_context.configure_mesh().x_labels(12).light_line_style(&WHITE).draw()?;
        let line_series = LineSeries::new(self.indicator.series.iter().map(|s| (s.date_time, s.value as f32)), &self.color);
        chart_context.draw_series(line_series)?;
        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        self.indicator.min_max()
    }
}
