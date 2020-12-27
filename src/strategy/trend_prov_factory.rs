use crate::technicals::ind_provider::IndicatorProvider;

use super::trend_provider::TrendProvider;

pub trait TrendProviderFactory<'a, T: TrendProvider> {
    fn create(ind_provider: &'a IndicatorProvider<'a>) -> T;
}
