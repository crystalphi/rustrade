use super::{
    indicator_plotter::{IndicatorPlotter, PlotterIndicatorContext},
    theme_plotter::ThemePlotter,
};
use crate::config::selection::Selection;
use ifmt::iformat;
use log::info;
use plotters::prelude::*;
use std::{path::Path, time::Instant};

pub struct Plotter<'a> {
    selection: Selection,
    plotters_ind: Vec<&'a dyn IndicatorPlotter>,
    plotters_ind_upper: Vec<&'a dyn PlotterIndicatorContext>,
    _plotters_ind_lower: Vec<&'a dyn PlotterIndicatorContext>,
}

impl<'a> Plotter<'a> {
    pub fn new(selection: Selection) -> Self {
        Plotter {
            selection,
            plotters_ind: vec![],
            plotters_ind_upper: vec![],
            _plotters_ind_lower: vec![],
        }
    }

    pub fn add_plotter_ind(&mut self, plotter_ind: &'a dyn IndicatorPlotter) {
        self.plotters_ind.push(plotter_ind);
    }

    pub fn add_plotter_upper_ind(&mut self, plotter_ind: &'a dyn PlotterIndicatorContext) {
        self.plotters_ind_upper.push(plotter_ind);
    }

    pub fn _add_plotter_lower_ind(&mut self, plotter_ind: &'a dyn PlotterIndicatorContext) {
        self._plotters_ind_lower.push(plotter_ind);
    }

    pub fn plot<P: AsRef<Path>>(&self, image_path: P) -> anyhow::Result<()> {
        let start = Instant::now();
        let symbol_minutes = &self.selection.candles_selection.symbol_minutes;

        let from_date = self.selection.candles_selection.start_time;
        let to_date = self.selection.candles_selection.end_time;

        let (min_price, max_price) = self
            .plotters_ind_upper
            .iter()
            .map(|i| i.min_max())
            .fold_first(|p, c| (p.0.min(c.0), p.1.max(c.1)))
            .unwrap();
        let min_price = min_price as f32;
        let max_price = max_price as f32;

        let (upper, lower) = {
            let root = BitMapBackend::new(&image_path, (1920, 1080)).into_drawing_area();
            root.split_vertically((80).percent())
        };

        let bg_color = ThemePlotter::back_ground();
        upper.fill(&bg_color)?;

        let font = FontDesc::new(FontFamily::Name("sans-serif"), 20.0, FontStyle::Normal).color(&ThemePlotter::fore_ground());

        let mut chart_context_upper = ChartBuilder::on(&upper)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            .caption(iformat!("{symbol_minutes.symbol} price"), font)
            .build_cartesian_2d(from_date..to_date, min_price..max_price)?;

        chart_context_upper.configure_mesh().x_labels(12).light_line_style(&bg_color).draw()?;

        for plotter_upper_ind in self.plotters_ind_upper.iter() {
            plotter_upper_ind.plot(&self.selection, &mut chart_context_upper)?;
        }

        lower.fill(&bg_color)?;

        for plotter_ind in self.plotters_ind.iter() {
            plotter_ind.plot(&self.selection, &upper, &lower)?;
        }

        // for plotters_ind_upper_ind in self.plotters_ind_lower.iter() {
        //     plotters_ind_upper_ind.plot(&mut chart_context_lower)?;
        // }

        info!("{}", iformat!("*** Plotting elapsed: {start.elapsed():?}"));
        Ok(())
    }
}
