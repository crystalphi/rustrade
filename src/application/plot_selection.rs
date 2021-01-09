use super::candles_provider::CandlesProvider;
use crate::{
    config::selection::Selection,
    tac_plotters::{
        candles_plotter::CandlePlotter, line_ind_plotter::LineIndicatorPlotter, macd_plotter::MacdPlotter, plotter::Plotter,
        topbottom_plotter::TopBottomPlotter,
    },
    technicals::technical::TechnicalIndicators,
    technicals::{ema_tac::EmaTac, macd::macd_tac::MacdTac, topbottom::TopBottomTac},
};
use colored::Colorize;
use ifmt::iformat;
use log::info;
use plotters::style::RGBColor;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

pub fn plot_selection(selection: Selection, mut candles_provider: Box<dyn CandlesProvider>) -> anyhow::Result<()> {
    let total_start = Instant::now();

    let candles_provider_clone = candles_provider.clone_provider();
    let candles = candles_provider.candles()?;

    let start_time = selection.candles_selection.start_time;
    let end_time = selection.candles_selection.end_time;
    let candles = candles
        .par_iter()
        .filter(|c| c.open_time >= start_time && c.open_time <= end_time)
        .collect::<Vec<_>>();
    info!(
        "Plotting selection {:?} {:?} candles.len {} image {}",
        selection.candles_selection.start_time,
        selection.candles_selection.end_time,
        candles.len(),
        selection.image_name.green()
    );

    let macd_tac = MacdTac::new(candles_provider_clone.clone_provider(), 34, 72, 17);
    let ema_short_tac = EmaTac::new(candles_provider_clone.clone_provider(), 17);
    let ema_long_tac = EmaTac::new(candles_provider_clone.clone_provider(), 72);
    let mut topbottom_tac = TopBottomTac::new(candles_provider_clone, 7);
    let topbottoms = topbottom_tac.topbottoms()?;
    let mut plotter = Plotter::new(selection.clone());

    // ema 17 = purple
    // ema 72 = orange

    let short_purple = RGBColor(128, 0, 128);
    let long_orange = RGBColor(255, 165, 0);
    // Upper indicators
    let candle_plotter = CandlePlotter::new(&candles);
    let topbottom_plotter = TopBottomPlotter::new(&topbottoms);
    let ema_short_plotter = LineIndicatorPlotter::new(ema_short_tac.main_indicator(), short_purple);
    let ema_long_plotter = LineIndicatorPlotter::new(ema_long_tac.main_indicator(), long_orange);

    plotter.add_plotter_upper_ind(&candle_plotter);
    plotter.add_plotter_upper_ind(&topbottom_plotter);
    plotter.add_plotter_upper_ind(&ema_short_plotter);
    plotter.add_plotter_upper_ind(&ema_long_plotter);

    // Lower indicators
    let macd_plotter = MacdPlotter::new(&macd_tac);
    plotter.add_plotter_ind(&macd_plotter);

    let start = Instant::now();
    plotter.plot(&selection.image_name)?;
    info!("{}", iformat!("### Plotting elapsed: {start.elapsed():?}"));

    info!("{}", iformat!("### Total plotting elapsed: {total_start.elapsed():?}"));

    Ok(())
}
