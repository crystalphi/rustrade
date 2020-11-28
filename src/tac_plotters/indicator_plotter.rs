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
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait PlotterIndicatorContext {
    fn plot(
        &self,
        chart_context: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
