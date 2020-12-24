use super::indicator_plotter::IndicatorPlotter;
use crate::{
    config::selection::Selection,
    technicals::{indicator::Indicator, macd::macd_tac::MacdTac},
};
use anyhow::anyhow;
use plotters::prelude::*;
use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, LabelAreaPosition, LineSeries},
    style::{BLACK, WHITE},
};
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
        selection: &Selection,
        upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> anyhow::Result<()> {
        let selected_tac = selection.tacs.get("macd").ok_or(anyhow!("Tac macd not selected!"))?;
        let mut selected_inds = Vec::new();
        for sel_ind_name in selected_tac.indicators {
            let tac_ind = self
                .macd_tac
                .indicators
                .get(&sel_ind_name)
                .ok_or(anyhow!("Indicator {} not found!", sel_ind_name))?;
            selected_inds.push(tac_ind);
        }
        plot_indicators(&selected_inds, selection, upper, lower)
    }
}
#![feature(fold_firstq)]
fn plot_indicators(
    indicators: &Vec<&Indicator>,
    selection: &Selection,
    _upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
) -> anyhow::Result<()> {
    let from_date = selection.candles_selection.start_time.unwrap();
    let to_date = selection.candles_selection.end_time.unwrap();

    let (min_macd, max_macd) = indicators
        .iter()
        .map(|i| i.min_max())
        .fold_first(|p, c| (p.0.min(c.0), p.1.max(c.1)))
        .ok_or(anyhow!("plot_indicators: have no min x max"))?;

    if min_macd == 0. && max_macd == 0. {
        return Err(anyhow!("plot_indicators: min x max values are zeros!"));
    }

    let mut cart_context_lower = ChartBuilder::on(&lower)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 80)
        .y_label_area_size(80)
        .x_label_area_size(30)
        //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, min_macd..max_macd)?;

    cart_context_lower.configure_mesh().light_line_style(&WHITE).draw()?;
    let macd_fast_series = LineSeries::new(indicator.series.iter().map(|t| (*t.date_time, t.value)), &BLACK);
    cart_context_lower.draw_series(macd_fast_series)?;
    Ok(())
}
