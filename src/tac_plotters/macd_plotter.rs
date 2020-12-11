use crate::{config::symbol_minutes::SymbolMinutes, technicals::macd::macd_tac::MacdTac, utils::str_to_datetime};
use chrono::{DateTime, Utc};
use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, LabelAreaPosition, LineSeries},
    style::{BLACK, WHITE},
};

use super::indicator_plotter::IndicatorPlotter;
use plotters::prelude::*;
use plotters_bitmap::{self, bitmap_pixel::RGBPixel};
pub struct MacdPlotter<'a> {
    macd_tac: &'a MacdTac<'a>,
}

impl<'a> MacdPlotter<'a> {
    pub fn new(macd_tac: &'a MacdTac<'a>) -> Self {
        MacdPlotter { macd_tac }
    }
}

impl<'a> IndicatorPlotter for MacdPlotter<'a> {
    fn plot(
        &self,
        _symbol_minutes: &SymbolMinutes,
        from_date: &DateTime<Utc>,
        to_date: &DateTime<Utc>,
        _upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let max_macd = self.macd_tac.macd.series.iter().fold(0f64, |acc, t| acc.max(t.value));

        let min_macd = self
            .macd_tac
            .macd
            .series
            .iter()
            .fold(max_macd, |acc, t| acc.min(t.value));

        if min_macd == 0. && max_macd == 0. {
            return Err("Valores estão zerado!".into());
        }

        let mut cart_context_lower = ChartBuilder::on(&lower)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(*from_date..*to_date, min_macd..max_macd)?;

        cart_context_lower.configure_mesh().light_line_style(&WHITE).draw()?;

        let macd_fast_series = LineSeries::new(
            self.macd_tac.macd.series.iter().map(|t| (t.date_time.clone(), t.value)),
            &BLACK,
        );

        cart_context_lower.draw_series(macd_fast_series)?;

        Ok(())
    }
}
