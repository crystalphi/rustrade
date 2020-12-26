use super::ind_type::IndicatorType;
use crate::technicals::indicator::Indicator;
use std::collections::HashMap;

pub struct IndicatorProvider<'a> {
    indicators: HashMap<IndicatorType, Indicator<'a>>,
}

impl<'a> IndicatorProvider<'a> {
    pub fn new() -> Self {
        Self { indicators: HashMap::new() }
    }

    pub fn indicator(&self, i_type: IndicatorType) -> anyhow::Result<&'a Indicator<'a>> {
        let ind = match self.indicators.get(i_type) {
            Some(ind) => {ind}
            None => {}
        }
        Ok(ind)
    }
}
