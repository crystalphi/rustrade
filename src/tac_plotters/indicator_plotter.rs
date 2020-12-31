use crate::config::selection::Selection;
use chrono::{DateTime, Utc};
use plotters::{
    coord::{types::RangedCoordf32, Shift},
    prelude::{Cartesian2d, ChartContext, DrawingArea, RangedDateTime},
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};

pub trait IndicatorPlotter {
    fn plot(
        &self,
        selection: &Selection,
        upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> anyhow::Result<()>;
}

pub trait PlotterIndicatorContext {
    fn plot(
        &self,
        selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()>;

    fn min_max(&self) -> (f64, f64);
}
