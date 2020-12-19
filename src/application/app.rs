use std::{thread, time};

use crate::{
    checker::Checker,
    config::{candles_selection::CandlesSelection, definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    model::candle::Candle,
    repository::Repository,
    technicals::{macd::macd_tac::MacdTac, technical::Technical},
    utils::str_to_datetime,
};

use super::{candles_provider::CandlesProvider, plot_selection::plot_from_selection};

pub struct Application<'a> {
    definition: ConfigDefinition,
    selection: Selection,
    candles_provider: CandlesProvider<'a>,
    synchronizer: &'a Checker<'a>,
}

impl<'a> Application<'a> {
    pub fn new(repo: &'a Repository, exchange: &'a Exchange, synchronizer: &'a Checker<'a>) -> Self {
        Application {
            synchronizer,
            candles_provider: CandlesProvider::new(repo, exchange),
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

    fn read_lines() -> Vec<String> {
        loop {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let result = line.trim_end_matches('\n').to_string();
            if !result.is_empty() {
                break result.lines().map(|l| l.to_string()).collect();
            }
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn run_stream(&mut self) {
        const GET_DEFINITION: &str = "GetDefinition";
        const GET_SELECTION: &str = "GetSelection";
        const SET_SELECTION: &str = "SetSelection";
        const END_SELECTION: &str = "EndSelection";

        const IMPORT: &str = "Import";
        const CHECK: &str = "Check";
        const PLOTT: &str = "Plott";

        const TERMINATE: &str = "Terminate";

        let mut candles: Vec<Candle> = Vec::new();

        let mut in_selection = false;
        let mut selection_buffer = String::from("");
        loop {
            for line in Self::read_lines() {
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
                    println!("set selection...");
                    in_selection = true;
                    continue;
                }

                if line == END_SELECTION {
                    println!(
                        "end selection... in_selection = {} selection_buffer.len() = {}",
                        in_selection,
                        selection_buffer.len()
                    );
                    in_selection = false;
                    self.set_selection(Selection::from_json(&selection_buffer));
                    selection_buffer.clear();
                    continue;
                }

                if in_selection {
                    selection_buffer.push_str(&line);
                    continue;
                }

                if line == IMPORT {
                    println!("getting candles...");
                    candles = self.candles_provider.candles_selection(self.selection.clone()).unwrap();
                    println!("candles got");
                    continue;
                }

                if line == PLOTT {
                    println!("plotting...");
                    let candles_ref = candles.iter().collect::<Vec<_>>();
                    plot_from_selection(&self.selection, candles_ref.as_slice());
                    println!("plotted!");
                    continue;
                }

                if line == CHECK {
                    println!("checking...");
                    self.synchronizer.check_inconsist();
                    println!("checked!");
                    continue;
                }

                println!("Unknown command \"{}\"", line);
            }
        }
    }
}
