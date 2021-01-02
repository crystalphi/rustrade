use super::{candles_provider::CandlesProvider, plot_selection::plot_selection, streamer::Streamer};
use crate::{
    checker::Checker,
    config::{definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    repository::Repository,
    strategy::{topbottom_triangle::topbottom_triangle, trader::run_trader_back_test},
    technicals::topbottom::TopBottomTac,
    utils::datetime_to_filename,
};
use chrono::Duration;
use ifmt::iformat;
use log::info;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::time::Instant;

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

    pub fn run_back_test(&mut self) -> anyhow::Result<()> {
        run_trader_back_test(self)?;
        Ok(())
    }

    pub fn plot_triangles(&mut self) -> anyhow::Result<()> {
        let start = Instant::now();
        info!("Loading...");

        let candles = self.candles_provider.candles_selection(&self.selection.candles_selection)?;
        let candles = candles.iter().collect::<Vec<_>>();
        let candles_ref = candles.as_slice();

        info!("{}", iformat!("Loaded {start.elapsed():?}"));

        let topbottoms = TopBottomTac::new(candles_ref, 7).topbottoms();
        let topbottoms = topbottoms.iter().collect::<Vec<_>>();
        let topbottoms_ref = topbottoms.as_slice();

        let minutes = self.selection.candles_selection.symbol_minutes.minutes;

        let triangles = topbottom_triangle(topbottoms_ref, &minutes);
        triangles.par_iter().for_each(|triangle| {
            let mut selection = self.selection.clone();
            let open_time = triangle.open(&minutes);
            let margin = Duration::minutes(minutes as i64 * 100);
            selection.candles_selection.start_time = Some(open_time - margin);
            selection.candles_selection.end_time = Some(open_time + margin);
            selection.image_name = format!("out/triangle_{}.png", datetime_to_filename(&open_time));
            info!("Plotting triangle {}", selection.image_name);
            plot_selection(&selection, candles_ref).unwrap();
        });
        Ok(())
    }

    pub fn run_stream(&'a mut self) -> anyhow::Result<()> {
        let mut streamer = Streamer::new(self);
        streamer.run()
    }

    pub fn plot_selection(&mut self) -> anyhow::Result<()> {
        let candles = self.candles_provider.candles_selection(&self.selection.candles_selection).unwrap();
        let candles = candles.iter().collect::<Vec<_>>();
        let candles_ref = candles.as_slice();

        plot_selection(&self.selection, candles_ref)
    }
}
