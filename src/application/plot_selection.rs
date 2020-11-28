use crate::{
    config::selection::Selection, model::candle::Candle, tac_plotters::plotter::plot_candles,
    technicals::macd::macd_tac::MacdTac, technicals::pivots::PivotTac,
};

pub fn plot_from_selection(selection: &Selection, candles: &[&Candle], image_name: &str) {
    //let candles_ref: Vec<_> = candles.iter().collect();

    let macd_tac = MacdTac::new(candles);

    let pivots = PivotTac::new(candles).pivots();

    plot_candles(
        &selection.symbol,
        &selection.minutes,
        &candles,
        &pivots,
        &macd_tac,
        &image_name,
    )
    .unwrap();
}
