use crate::{model::candle::Candle, technicals::pivots::Pivot, utils::str_to_datetime};
use chrono::{DateTime, Duration, Utc};
use ifmt::iformat;
use plotters::prelude::*;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::path::Path;

use super::{
    candles_plotter::CandlePlotter, indicator_plotter::IndicatorPlotter,
    indicator_plotter::PlotterIndicatorContext, pivot_plotter::PivotPlotter,
};

pub struct Plotter<'a> {
    candles: &'a [&'a Candle],
    plotters_ind: Vec<&'a dyn IndicatorPlotter>,
    plotters_ind_upper: Vec<&'a dyn PlotterIndicatorContext>,
    plotters_ind_lower: Vec<&'a dyn PlotterIndicatorContext>,
}

impl<'a> Plotter<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        Plotter {
            candles,
            plotters_ind: vec![],
            plotters_ind_upper: vec![],
            plotters_ind_lower: vec![],
        }
    }

    pub fn add_plotter_ind(&mut self, plotter_ind: &'a dyn IndicatorPlotter) {
        self.plotters_ind.push(plotter_ind);
    }

    pub fn add_plotter_upper_ind(&mut self, plotter_ind: &'a dyn PlotterIndicatorContext) {
        self.plotters_ind_upper.push(plotter_ind);
    }

    pub fn add_plotter_lower_ind(&mut self, plotter_ind: &'a dyn PlotterIndicatorContext) {
        self.plotters_ind_lower.push(plotter_ind);
    }

    pub fn plot<P: AsRef<Path>>(
        &self,
        symbol: &str,
        minutes: &i64,
        image_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (min_price, max_price) = prices_range_from_candles(&self.candles);
        let (from_date, to_date) = date_time_range_from_candles(&self.candles, minutes);

        let (upper, lower) = {
            let root = BitMapBackend::new(&image_path, (1920, 1080)).into_drawing_area();
            root.split_vertically((80).percent())
        };
        upper.fill(&WHITE)?;

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

        for plotter_upper_ind in self.plotters_ind_upper.iter() {
            plotter_upper_ind.plot(&mut chart_context)?;
        }

        lower.fill(&WHITE)?;

        for plotter_lower_ind in self.plotters_ind_lower.iter() {
            plotter_lower_ind.plot(&mut chart_context)?;
        }

        for plotter_ind in self.plotters_ind.iter() {
            plotter_ind.plot(symbol, minutes, &from_date, &to_date, &upper, &lower)?;
        }

        Ok(())
    }
}

pub fn date_time_range_from_candles(
    candles: &[&Candle],
    minutes: &i64,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let from_date = str_to_datetime(&candles[0].close_time) - Duration::minutes(*minutes as i64);
    let to_date = str_to_datetime(&candles[candles.len() - 1].close_time)
        + Duration::minutes(*minutes as i64);
    (from_date, to_date)
}

pub fn prices_range_from_candles(candles: &[&Candle]) -> (Decimal, Decimal) {
    let max_price = candles.iter().fold(dec!(0), |acc, x| acc.max(x.high));
    let min_price = candles.iter().fold(max_price, |acc, x| acc.min(x.low));
    (min_price, max_price)
}

pub fn plot_candles<'a>(
    symbol: &str,
    minutes: &i64,
    candles: &'a [&'a Candle],
    pivots: &'a [Pivot],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut plotter = Plotter::new(candles);

    let candle_plotter = CandlePlotter::new(candles);

    let pivot_plotter = PivotPlotter::new(pivots);

    plotter.add_plotter_upper_ind(&candle_plotter);
    plotter.add_plotter_upper_ind(&pivot_plotter);

    plotter.plot(symbol, minutes, "out/stock.png")?;

    Ok(())
}

// pub fn plot_candles(
//     symbol: &str,
//     minutes: &u32,
//     candles: &[&Candle],
//     pivots: &[Pivot],
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let red = RGBColor(164, 16, 64);
//     let green = RGBColor(16, 196, 64);

//     let start = Instant::now();

//     let (upper, lower) = {
//         let root = BitMapBackend::new("out/stock.png", (1920, 1080)).into_drawing_area();
//         root.split_vertically((80).percent())
//     };
//     let from_date = str_to_datetime(&candles[0].close_time) - Duration::minutes(*minutes as i64);

//     let to_date = str_to_datetime(&candles[candles.len() - 1].close_time)
//         + Duration::minutes(*minutes as i64);

//     {
//         upper.fill(&WHITE)?;

//         let max_price = candles.iter().fold(dec!(0), |acc, x| acc.max(x.high));
//         let min_price = candles.iter().fold(max_price, |acc, x| acc.min(x.low));

//         let min_price = min_price.to_f32().unwrap();
//         let max_price = max_price.to_f32().unwrap();

//         let mut chart_context = ChartBuilder::on(&upper)
//             .set_label_area_size(LabelAreaPosition::Left, 30)
//             .set_label_area_size(LabelAreaPosition::Right, 80)
//             .y_label_area_size(80)
//             .x_label_area_size(30)
//             .caption(iformat!("{symbol} price"), ("sans-serif", 20.0).into_font())
//             .build_cartesian_2d(from_date..to_date, min_price..max_price)?;

//         chart_context
//             .configure_mesh()
//             .x_labels(12)
//             .light_line_style(&WHITE)
//             .draw()?;

//         let candle_series = candles.iter().map(|x| {
//             CandleStick::new(
//                 str_to_datetime(&x.close_time),
//                 x.open.to_f32().unwrap(),
//                 x.high.to_f32().unwrap(),
//                 x.low.to_f32().unwrap(),
//                 x.close.to_f32().unwrap(),
//                 &green,
//                 &red,
//                 2,
//             )
//         });
//         chart_context.draw_series(candle_series)?;

//         let low_pivots = PointSeries::of_element(
//             pivots
//                 .iter()
//                 .filter(|p| p.type_p == PivotType::Low)
//                 .map(|c| (str_to_datetime(&c.close_time), c.price.to_f32().unwrap())),
//             3,
//             ShapeStyle::from(&red).filled(),
//             &|coord, size, style| {
//                 EmptyElement::at(coord) + Circle::new((0, 0), size, style)
//                 //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
//             },
//         );
//         chart_context.draw_series(low_pivots)?;

//         let high_pivots = PointSeries::of_element(
//             pivots
//                 .iter()
//                 .filter(|p| p.type_p == PivotType::High)
//                 .map(|c| (str_to_datetime(&c.close_time), c.price.to_f32().unwrap())),
//             3,
//             ShapeStyle::from(&green).filled(),
//             &|coord, size, style| {
//                 EmptyElement::at(coord) + Circle::new((0, 0), size, style)
//                 //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
//             },
//         );

//         chart_context.draw_series(high_pivots)?;
//     };

//     {
//         lower.fill(&WHITE)?;

//         let max_macd = candles.iter().fold(0f64, |acc, t| acc.max(t.macd));
//         let min_macd = candles.iter().fold(max_macd, |acc, t| acc.min(t.macd));
//         let min_macd = min_macd.to_f32().unwrap();
//         let max_macd = max_macd.to_f32().unwrap();

//         iprintln!("min_macd: {min_macd} max_macd: {max_macd}");

//         let mut cart_context = ChartBuilder::on(&lower)
//             .set_label_area_size(LabelAreaPosition::Left, 30)
//             .set_label_area_size(LabelAreaPosition::Right, 80)
//             .y_label_area_size(80)
//             .x_label_area_size(30)
//             //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
//             .build_cartesian_2d(from_date..to_date, min_macd..max_macd)?;

//         cart_context
//             .configure_mesh()
//             .light_line_style(&WHITE)
//             .draw()?;

//         let macd_fast_series = LineSeries::new(
//             candles
//                 .iter()
//                 .map(|t| (str_to_datetime(&t.candle.close_time), t.macd as f32)),
//             &BLACK,
//         );

//         cart_context.draw_series(macd_fast_series)?;
//     }
//     //iprintln!("Plotting {macd_tacs.len()} pivots {pivots.len()} : {start.elapsed():?}");

//     Ok(())
// }
