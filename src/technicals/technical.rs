use std::collections::HashMap;

use crate::config::definition::TacDefinition;

use super::indicator::Indicator;

pub trait TechnicalDefinition {
    fn definition() -> TacDefinition;
}

pub trait TechnicalIndicators {
    fn indicators(&self) -> &HashMap<String, Indicator>;
    fn main_indicator(&self) -> &Indicator;
    // fn clone(&self) -> Self;
    // fn clone_from(&mut self, source: &Self);
}
