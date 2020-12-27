use crate::technicals::ind_provider::IndicatorProvider;

use super::trend_provider::TrendProvider;

pub trait TrendProviderFactory<'a, T: TrendProvider<'a>> {
    fn create(ind_provider: IndicatorProvider<'a>) -> T;
}
