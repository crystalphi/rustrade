use std::time::Instant;

use chrono::Duration;
use ifmt::{iformat, iprintln};
use plotters::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

use crate::{
    analyzers::macd_tac::MacdCandle, analyzers::pivots::Pivot, analyzers::pivots::PivotType,
    utils::str_to_datetime,
};

pub fn plot_tecals(
    symbol: &str,
    minutes: &u32,
    macd_tacs: &[MacdCandle],
    pivots: &[Pivot],
) -> Result<(), Box<dyn std::error::Error>> {
    let red = RGBColor(164, 16, 64);
    let green = RGBColor(16, 196, 64);

    let start = Instant::now();

    let (upper, lower) = {
        let root = BitMapBackend::new("out/stock.png", (1920, 1080)).into_drawing_area();
        root.split_vertically((80).percent())
    };
    let from_date =
        str_to_datetime(&macd_tacs[0].candle.close_time) - Duration::minutes(*minutes as i64);

    let to_date = str_to_datetime(&macd_tacs[macd_tacs.len() - 1].candle.close_time)
        + Duration::minutes(*minutes as i64);

    {
        upper.fill(&WHITE)?;

        let max_price = macd_tacs
            .iter()
            .fold(dec!(0), |acc, x| acc.max(x.candle.high));
        let min_price = macd_tacs
            .iter()
            .fold(max_price, |acc, x| acc.min(x.candle.low));

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

        let candle_series = macd_tacs.iter().map(|x| {
            CandleStick::new(
                str_to_datetime(&x.candle.close_time),
                x.candle.open.to_f32().unwrap(),
                x.candle.high.to_f32().unwrap(),
                x.candle.low.to_f32().unwrap(),
                x.candle.close.to_f32().unwrap(),
                &green,
                &red,
                2,
            )
        });
        chart_context.draw_series(candle_series)?;

        let low_pivots = PointSeries::of_element(
            pivots
                .iter()
                .filter(|p| p.type_p == PivotType::Low)
                .map(|c| (str_to_datetime(&c.close_time), c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&red).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );
        chart_context.draw_series(low_pivots)?;

        let high_pivots = PointSeries::of_element(
            pivots
                .iter()
                .filter(|p| p.type_p == PivotType::High)
                .map(|c| (str_to_datetime(&c.close_time), c.price.to_f32().unwrap())),
            3,
            ShapeStyle::from(&green).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        );

        chart_context.draw_series(high_pivots)?;
    };

    {
        lower.fill(&WHITE)?;

        let max_macd = macd_tacs.iter().fold(0f64, |acc, t| acc.max(t.macd));
        let min_macd = macd_tacs.iter().fold(max_macd, |acc, t| acc.min(t.macd));
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
            macd_tacs
                .iter()
                .map(|t| (str_to_datetime(&t.candle.close_time), t.macd as f32)),
            &BLACK,
        );

        cart_context.draw_series(macd_fast_series)?;
    }
    iprintln!("Plotting {macd_tacs.len()} pivots {pivots.len()} : {start.elapsed():?}");

    Ok(())
}
