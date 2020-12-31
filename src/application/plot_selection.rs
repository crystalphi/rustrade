use crate::{
    config::selection::Selection,
    model::candle::Candle,
    tac_plotters::{
        candles_plotter::CandlePlotter, line_ind_plotter::LineIndicatorPlotter, macd_plotter::MacdPlotter, plotter::Plotter,
        topbottom_plotter::TopBottomPlotter,
    },
    technicals::ind_provider::IndicatorProvider,
    technicals::technical::TechnicalIndicators,
    technicals::{ema_tac::EmaTac, macd::macd_tac::MacdTac, topbottom::TopBottomTac},
};
use ifmt::iformat;
use log::info;
use plotters::style::RGBColor;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

pub fn plot_selection(selection: &Selection, candles: &[&Candle]) -> anyhow::Result<()> {
    let start_time = selection.candles_selection.start_time.unwrap();
    let end_time = selection.candles_selection.end_time.unwrap();
    let candles = candles
        .par_iter()
        .filter(|c| c.open_time >= start_time && c.open_time <= end_time)
        .copied()
        .collect::<Vec<_>>();
    info!(
        "Plotting selection {:?} {:?} candles.len {}",
        selection.candles_selection.start_time,
        selection.candles_selection.end_time,
        candles.len()
    );

    let mut _indicator_provider = IndicatorProvider::new();

    let macd_tac = MacdTac::new(&candles, 34, 72, 17);
    let ema_short_tac = EmaTac::new(&candles, 17);
    let ema_long_tac = EmaTac::new(&candles, 72);
    let topbottoms = TopBottomTac::new(&candles, 7).topbottoms();

    let mut plotter = Plotter::new(selection);

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

    Ok(())
}
