use std::collections::HashMap;

use crate::config::definition::TacDefinition;

use super::indicator::Indicator;

pub trait TechnicalDefinition<'a> {
    fn definition() -> TacDefinition;
}

pub trait TechnicalIndicators<'a> {
    fn indicators(&self) -> &HashMap<String, Indicator<'a>>;
    fn main_indicator(&self) -> &Indicator;
}
