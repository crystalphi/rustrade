use crate::{
    config::selection::Selection, model::candle::Candle, tac_plotters::plotter::plot_candles,
    technicals::macd::macd_tac::MacdTac, technicals::pivots::PivotTac,
};

pub fn plot_from_selection(selection: &Selection, candles: &[&Candle]) {
    let macd_tac = MacdTac::new(candles);

    let pivots = PivotTac::new(candles).pivots();

    plot_candles(
        &selection.candles_selection.symbol_minutes,
        &candles,
        &pivots,
        &macd_tac,
        &selection.image_name,
    )
    .unwrap();
}
