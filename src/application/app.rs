use crate::{
    config::{candles_selection::CandlesSelection, definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    provider::candles_buffer::CandlesBuffer,
    repository::Repository,
    technicals::{macd::macd_tac::MacdTac, technical::Technical},
    utils::str_to_datetime,
};

use super::{candles_provider::CandlesProvider, plot_selection::plot_from_selection};

pub struct Application<'a> {
    definition: ConfigDefinition,
    selection: Selection,
    candles_provider: CandlesProvider<'a>,
}

impl<'a> Application<'a> {
    pub fn new(repo: &'a Repository, exchange: &'a Exchange, candles_buffer: &'a mut CandlesBuffer) -> Self {
        Application {
            candles_provider: CandlesProvider::new(candles_buffer, repo, exchange),
            selection: Selection {
                tacs: vec![MacdTac::definition()],
                candles_selection: CandlesSelection::new(
                    "BTCUSDT",
                    &15u32,
                    Some(&str_to_datetime("2020-10-01 00:00:00")),
                    Some(&str_to_datetime("2020-11-30 00:00:00")),
                ),

                image_name: "out/stock.png".to_string(),
            },
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

    fn read_line() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end_matches('\n').to_string()
    }

    pub fn run_stream(&mut self) {
        const GET_DEFINITION: &str = "GetDefinition";
        const GET_SELECTION: &str = "GetSelection";
        const SET_SELECTION: &str = "SetSelection";
        const TERMINATE: &str = "Terminate";

        loop {
            let mut line = Self::read_line();

            if line == TERMINATE {
                println!("Terminated!");
                break;
            }
            if line == GET_DEFINITION {
                println!("{}", self.definition.to_json());
                continue;
            }
            if line == GET_SELECTION {
                println!("{}", self.selection.to_json());
                continue;
            }
            if line == SET_SELECTION {
                line = Self::read_line();
                self.set_selection(Selection::from_json(&line));

                let candles = self.candles_provider.candles_selection(self.selection.clone()).unwrap();
                let candles_ref = candles.iter().collect::<Vec<_>>();

                plot_from_selection(&self.selection, candles_ref.as_slice());
                continue;
            }
            println!("Unknown command {}", line);
        }
    }
}
