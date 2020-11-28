use crate::{
    config::{definition::ConfigDefinition, selection::Selection},
    technicals::{macd::macd_tac::MacdTac, technical::Technical},
};

pub struct App {
    definition: ConfigDefinition,
    selection: Selection,
}

impl App {
    pub fn new() -> Self {
        App {
            selection: Selection {
                tacs: vec![MacdTac::definition()],
                minutes: 15u32,
                symbol: "BTCUSDT".to_string(),
                period_start: "2020-10-01 00:00:00".to_string(),
                period_end: "2020-11-30 00:00:00".to_string(),
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
        line
    }

    pub fn run_stream(&mut self) {
        const GET_DEFINITION: &str = "GetDefinition";
        const GET_SELECTION: &str = "GetSelection";
        const SET_SELECTION: &str = "SetSelection";
        const TERMINATE: &str = "Terminate";

        loop {
            let mut line = Self::read_line();

            if line == TERMINATE {
                break;
            }
            if line == GET_DEFINITION {
                println!("{}", self.definition.to_json());
            }
            if line == GET_SELECTION {
                println!("{}", self.selection.to_json());
            }
            if line == SET_SELECTION {
                line = Self::read_line();
                self.set_selection(Selection::from_json(&line));
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
