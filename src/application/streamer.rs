use super::{app::Application, candles_provider::CandlesProvider};
use crate::config::selection::Selection;
use core::time;
use log::info;
use std::thread;

pub struct Streamer<'a> {
    app: &'a mut Application<'a>,
}

impl<'a> Streamer<'a> {
    pub fn new(app: &'a mut Application<'a>) -> Self {
        Self { app }
    }

    fn read_lines() -> anyhow::Result<Vec<String>> {
        loop {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            let result = line.trim_end_matches('\n').to_string();
            if !result.is_empty() {
                break Ok(result.lines().map(|l| l.to_string()).collect());
            }
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        const GET_DEFINITION: &str = "GetDefinition";
        const GET_SELECTION: &str = "GetSelection";
        const SET_SELECTION: &str = "SetSelection";
        const END_SELECTION: &str = "EndSelection";

        const IMPORT: &str = "Import";
        const CHECK: &str = "Check";
        const PLOT: &str = "Plot";

        const TERMINATE: &str = "Terminate";

        let mut in_selection = false;
        let mut selection_buffer = String::from("");
        'outer: loop {
            for line in Self::read_lines()? {
                if line == TERMINATE {
                    info!("Terminated!");
                    break 'outer;
                }

                if line == GET_DEFINITION {
                    info!("{}", self.app.definition.to_json());
                    continue;
                }

                if line == GET_SELECTION {
                    info!("{}", self.app.selection.to_json());
                    continue;
                }

                if line == SET_SELECTION {
                    info!("Set selection...");
                    in_selection = true;
                    continue;
                }

                if line == END_SELECTION {
                    info!(
                        "End selection... in_selection = {} selection_buffer.len() = {}",
                        in_selection,
                        selection_buffer.len()
                    );
                    in_selection = false;
                    self.app.set_selection(Selection::from_json(&selection_buffer));
                    selection_buffer.clear();
                    continue;
                }

                if in_selection {
                    selection_buffer.push_str(&line);
                    continue;
                }

                if line == IMPORT {
                    info!("Getting candles...");
                    self.app.candles_provider.set_candles_selection(self.app.selection.candles_selection.clone());
                    let _candles = self.app.candles_provider.candles()?;
                    info!("Candles got");
                    continue;
                }

                if line == PLOT {
                    info!("Plotting...");
                    self.app.plot_selection()?;
                    info!("Plotted!");
                    continue;
                }

                if line == CHECK {
                    info!("Checking...");
                    // TODO
                    //self.app.synchronizer.check_inconsist(&self.app.repo, &self.app.selection.candles_selection);
                    info!("Checked!");
                    continue;
                }

                info!("Unknown command \"{}\"", line);
            }
        }
        Ok(())
    }
}
