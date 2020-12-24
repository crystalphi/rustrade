use super::indicator_plotter::PlotterIndicatorContext;
use crate::{
    config::selection::Selection,
    model::candle::Candle,
    technicals::{ema_tac::EmaTac, indicator::Indicator},
};
use chrono::{DateTime, Utc};
use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_bitmap::bitmap_pixel::RGBPixel;

pub struct EmaPlotter<'a> {
    ema_ind: &'a Indicator<'a>,
}

impl<'a> EmaPlotter<'a> {
    pub fn new(ema_tac: &'a EmaTac<'a>) -> Self {
        let ema_ind = ema_tac.indicators.get("ema").unwrap();
        Self { ema_ind }
    }
}

impl<'a> PlotterIndicatorContext for EmaPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let orange = RGBColor(255, 165, 0);

        chart_context.configure_mesh().x_labels(12).light_line_style(&WHITE).draw()?;

        let ema_series = LineSeries::new(self.ema_ind.series.iter().map(|s| (*s.date_time, s.value as f32)), &orange);

        chart_context.draw_series(ema_series)?;
        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        self.ema_ind.min_max()
    }
}
