use chrono::{DateTime, Utc};
use rust_decimal_macros::dec;

use crate::model::candle::Candle;

use super::plotter::PlotterIndicator;

pub struct CandlePlotter<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> PlotterIndicator for CandlePlotter<'a> {
    fn plot(
        &self,
        from_date: &DateTime<Utc>,
        to_date: &DateTime<Utc>,
        minutes: &i64,
        upper: &plotters::prelude::DrawingArea<
            plotters::prelude::BitMapBackend,
            plotters::coord::Shift,
        >,
        lower: &plotters::prelude::DrawingArea<
            plotters::prelude::BitMapBackend,
            plotters::coord::Shift,
        >,
    ) {
        let max_price = self.candles.iter().fold(dec!(0), |acc, x| acc.max(x.high));
        let min_price = self.candles.iter().fold(max_price, |acc, x| acc.min(x.low));

        let min_price = min_price.to_f32().unwrap();
        let max_price = max_price.to_f32().unwrap();

        let mut chart_context = ChartBuilder::on(&upper)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            .caption(iformat!("{symbol} price"), ("sans-serif", 20.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_price..max_price)?;

        chart_context
            .configure_mesh()
            .x_labels(12)
            .light_line_style(&WHITE)
            .draw()?;

        let candle_series = self.candles.iter().map(|x| {
            CandleStick::new(
                str_to_datetime(&x.close_time),
                x.open.to_f32().unwrap(),
                x.high.to_f32().unwrap(),
                x.low.to_f32().unwrap(),
                x.close.to_f32().unwrap(),
                &green,
                &red,
                2,
            )
        });
        chart_context.draw_series(candle_series)?;
    }
}
