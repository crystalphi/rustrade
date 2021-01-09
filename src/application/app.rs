use super::{
    candles_provider::{CandlesProvider, CandlesProviderBuffer, CandlesProviderBufferSingleton, CandlesProviderSelection},
    plot_selection::plot_selection,
    streamer::Streamer,
};
use crate::{
    checker::Checker,
    config::{definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    repository::Repository,
    strategy::{topbottom_triangle::topbottom_triangle, trader::run_trader_back_test},
    technicals::topbottom::TopBottomTac,
};
use chrono::Duration;
use std::{cell::RefCell, rc::Rc};

pub struct Application<'a> {
    pub definition: ConfigDefinition,
    pub selection: Selection,
    pub candles_provider: CandlesProviderBuffer,
    pub synchronizer: &'a Checker<'a>,
}

impl<'a> Application<'a> {
    pub fn new(repository: Repository, exchange: Exchange, synchronizer: &'a Checker<'a>, selection: Selection) -> Self {
        let candles_provider_singleton = CandlesProviderBufferSingleton::new(repository, exchange);
        Application {
            synchronizer,
            candles_provider: CandlesProviderBuffer::new(Rc::new(RefCell::new(candles_provider_singleton))),
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
        let selection = self.selection.clone();
        let candles_selection = selection.candles_selection.clone();
        let candles_provider_selection = CandlesProviderSelection::new(self.candles_provider.clone(), candles_selection);
        let candles_provider = Box::new(candles_provider_selection);
        plot_triangles(selection, candles_provider)
    }

    pub fn run_stream(&'a mut self) -> anyhow::Result<()> {
        let mut streamer = Streamer::new(self);
        streamer.run()
    }

    pub fn plot_selection(&mut self) -> anyhow::Result<()> {
        let selection = self.selection.clone();
        let candles_provider_selection = CandlesProviderSelection::new(self.candles_provider.clone(), selection.candles_selection.clone());
        let candles_provider = Box::new(candles_provider_selection);
        plot_selection(selection, candles_provider)
    }
}

pub fn plot_triangles(selection: Selection, candles_provider: Box<dyn CandlesProvider>) -> anyhow::Result<()> {
    let mut topbottom_tac = TopBottomTac::new(candles_provider.clone_provider(), 7);
    let topbottoms = topbottom_tac.topbottoms()?;

    let topbottoms = topbottoms.iter().collect::<Vec<_>>();
    let topbottoms_ref = topbottoms.as_slice();

    let minutes = selection.candles_selection.symbol_minutes.minutes;

    let triangles = topbottom_triangle(topbottoms_ref, &minutes);
    triangles.iter().for_each(|triangle| {
        let mut selection_par = selection.clone();
        let open_time = triangle.open(&minutes);
        let margin = Duration::minutes(minutes as i64 * 100);
        selection_par.candles_selection.start_time = Some(open_time - margin);
        selection_par.candles_selection.end_time = Some(open_time + margin);
        //selection.image_name = format!("out/triangle_{}.png", datetime_to_filename(&open_time));
        //info!("Plotting triangle {}", selection.image_name);

        plot_selection(selection_par, candles_provider.clone_provider()).unwrap();
    });
    Ok(())
}
