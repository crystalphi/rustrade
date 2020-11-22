use std::time::Instant;

use chrono::Duration;
use ifmt::{iformat, iprintln};
use plotters::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

use crate::{analyzer::Technical, utils::str_to_datetime};

pub fn plot_tecals(
    symbol: &str,
    minutes: &u32,
    technicals: &[Technical],
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let (upper, lower) = {
        let root = BitMapBackend::new("out/stock.png", (1024, 768)).into_drawing_area();
        root.split_vertically((80).percent())
    };
    let from_date =
        str_to_datetime(&technicals[0].candle.close_time) - Duration::minutes(*minutes as i64);

    let to_date = str_to_datetime(&technicals[technicals.len() - 1].candle.close_time)
        + Duration::minutes(*minutes as i64); /* */

    {
        upper.fill(&WHITE)?;

        let max_price = technicals
            .iter()
            .fold(dec!(0), |acc, x| acc.max(x.candle.high));
        let min_price = technicals
            .iter()
            .fold(max_price, |acc, x| acc.min(x.candle.low));

        let min_price = min_price.to_f32().unwrap();
        let max_price = max_price.to_f32().unwrap();

        let mut chart_candles = ChartBuilder::on(&upper)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            .caption(iformat!("{symbol} price"), ("sans-serif", 20.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_price..max_price)?;

        chart_candles
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()?;

        let candle_series = technicals.iter().map(|x| {
            CandleStick::new(
                str_to_datetime(&x.candle.close_time),
                x.candle.open.to_f32().unwrap(),
                x.candle.high.to_f32().unwrap(),
                x.candle.low.to_f32().unwrap(),
                x.candle.close.to_f32().unwrap(),
                &GREEN,
                &RED,
                4,
            )
        });
        chart_candles.draw_series(candle_series)?
    };

    {
        lower.fill(&WHITE)?;

        let max_macd = technicals.iter().fold(0f64, |acc, t| acc.max(t.macd));
        let min_macd = technicals.iter().fold(max_macd, |acc, t| acc.min(t.macd));
        let min_macd = min_macd.to_f32().unwrap();
        let max_macd = max_macd.to_f32().unwrap();

        iprintln!("min_macd: {min_macd} max_macd: {max_macd}");

        let mut chart_candles = ChartBuilder::on(&lower)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_macd..max_macd)?;

        chart_candles
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()?;

        let macd_fast_series = LineSeries::new(
            technicals
                .iter()
                .map(|t| (str_to_datetime(&t.candle.close_time), t.macd as f32)),
            &BLACK,
        );

        chart_candles.draw_series(macd_fast_series)?;
    }
    iprintln!("Plotting {technicals.len()}: {start.elapsed():?}");

    Ok(())
}
