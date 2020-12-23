use chrono::{DateTime, Utc};
use plotters::{
    coord::{types::RangedCoordf32, Shift},
    prelude::{Cartesian2d, ChartContext, DrawingArea, RangedDateTime},
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};

use crate::config::symbol_minutes::SymbolMinutes;

pub trait IndicatorPlotter {
    fn plot(
        &self,
        symbol_minutes: &SymbolMinutes,
        from_date: &DateTime<Utc>,
        to_date: &DateTime<Utc>,
        upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> anyhow::Result<()>;
}

pub trait PlotterIndicatorContext {
    fn plot(
        &self,
        from_date: &DateTime<Utc>,
        to_date: &DateTime<Utc>,

        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()>;
}
