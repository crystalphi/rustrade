use super::trend_provider::TrendProvider;
use crate::technicals::ind_provider::IndicatorProvider;

pub trait TrendProviderFactory<'a, T: TrendProvider<'a>> {
    fn create(ind_provider: IndicatorProvider<'a>) -> T;
}
