use super::indicator_plotter::IndicatorPlotter;

pub struct MacdPlotter {}

impl IndicatorPlotter for MacdPlotter {
    fn plot(
        &self,
        symbol: &str,
        minutes: &i64,
        from_date: &chrono::DateTime<chrono::Utc>,
        to_date: &chrono::DateTime<chrono::Utc>,
        upper: &plotters::prelude::DrawingArea<
            plotters_bitmap::BitMapBackend<plotters_bitmap::bitmap_pixel::RGBPixel>,
            plotters::coord::Shift,
        >,
        lower: &plotters::prelude::DrawingArea<
            plotters_bitmap::BitMapBackend<plotters_bitmap::bitmap_pixel::RGBPixel>,
            plotters::coord::Shift,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let max_macd = candles.iter().fold(0f64, |acc, t| acc.max(t.macd));
        let min_macd = candles.iter().fold(max_macd, |acc, t| acc.min(t.macd));
        let min_macd = min_macd.to_f32().unwrap();
        let max_macd = max_macd.to_f32().unwrap();

        iprintln!("min_macd: {min_macd} max_macd: {max_macd}");

        let mut cart_context = ChartBuilder::on(&lower)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_macd..max_macd)?;

        cart_context
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()?;

        let macd_fast_series = LineSeries::new(
            candles
                .iter()
                .map(|t| (str_to_datetime(&t.candle.close_time), t.macd as f32)),
            &BLACK,
        );

        cart_context.draw_series(macd_fast_series)?;

        Ok(())
    }
}
