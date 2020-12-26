use std::collections::HashMap;

use crate::config::definition::TacDefinition;

use super::indicator::Indicator;

pub trait Technical<'a> {
    fn definition() -> TacDefinition;
    fn indicators(&self) -> &HashMap<String, Indicator<'a>>;
    fn main_indicator(&self) -> &Indicator;
}
