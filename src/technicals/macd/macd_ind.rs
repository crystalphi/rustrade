use crate::technicals::{indicator::Indicator, serie::Serie};

pub struct MacdInd<'a> {
    series: Vec<Serie<'a>>,
}
impl<'a> MacdInd<'a> {
    pub fn new() -> Self {
        MacdInd { series: vec![] }
    }
}

impl<'a> Indicator<'a> for MacdInd<'a> {
    fn name() -> String {
        "MACD".into()
    }

    fn series(&'a self) -> &'a Vec<Serie<'a>> {
        &self.series
    }
}
