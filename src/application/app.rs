use std::time::Instant;

use super::{candles_provider::CandlesProvider, streamer::Streamer};
use crate::{
    checker::Checker,
    config::{definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    model::candle::Candle,
    repository::Repository,
    strategy::pivots_triangle::pivots_triangle,
    tac_plotters::plotter::plot_candles,
    technicals::macd::macd_tac::MacdTac,
    technicals::pivots::PivotTac,
    utils::datetime_to_filename,
};
use chrono::Duration;
use ifmt::iformat;
use log::info;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

pub struct Application<'a> {
    pub repo: &'a Repository,
    pub exchange: &'a Exchange,
    pub definition: ConfigDefinition,
    pub selection: Selection,
    pub candles_provider: CandlesProvider<'a>,
    pub synchronizer: &'a Checker<'a>,
}

impl<'a> Application<'a> {
    pub fn new(repo: &'a Repository, exchange: &'a Exchange, synchronizer: &'a Checker<'a>, selection: Selection) -> Self {
        Application {
            repo,
            exchange,
            synchronizer,
            candles_provider: CandlesProvider::new(repo, exchange),
            selection,
            definition: ConfigDefinition::new(),
        }
    }

    pub fn definition(&self) -> &ConfigDefinition {
        &self.definition
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    pub fn plot_selection(selection: &Selection, candles: &[&Candle]) {
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
        let macd_tac = MacdTac::new(&candles);
        let pivots = PivotTac::new(&candles).pivots();
        plot_candles(&selection, &candles, &pivots, &macd_tac, &selection.image_name).unwrap();
    }

    pub fn plot_triangles(&mut self) {
        let start = Instant::now();
        info!("Loading...");

        let candles = self.candles_provider.candles_selection(&self.selection).unwrap();
        let candles = candles.iter().collect::<Vec<_>>();
        let candles_ref = candles.as_slice();

        info!("{}", iformat!("Loaded {start.elapsed():?}"));

        let pivots = PivotTac::new(candles_ref).pivots();
        let pivots = pivots.iter().collect::<Vec<_>>();
        let pivots_ref = pivots.as_slice();

        let minutes = self.selection.candles_selection.symbol_minutes.minutes;

        let triangles = pivots_triangle(pivots_ref, &minutes);
        // let triangles = triangles.iter().collect::<Vec<_>>();
        // let triangles = triangles.as_slice();
        triangles.par_iter().for_each(|triangle| {
            let mut selection = self.selection.clone();
            let open_time = triangle.open(&minutes);
            let margin = Duration::minutes(minutes as i64 * 100);
            selection.candles_selection.start_time = Some(open_time - margin);
            selection.candles_selection.end_time = Some(open_time + margin);
            selection.image_name = format!("out/triangle_{}.png", datetime_to_filename(&open_time));
            info!("Plotting triangle {}", selection.image_name);
            Self::plot_selection(&selection, candles_ref);
        });
    }

    pub fn run_stream(&'a mut self) {
        let mut streamer = Streamer::new(self);
        streamer.run();
    }
}
