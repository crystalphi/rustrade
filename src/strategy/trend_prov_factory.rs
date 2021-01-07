use super::trend_provider::TrendProvider;
use crate::technicals::ind_provider::IndicatorProvider;

pub trait TrendProviderFactory<T: TrendProvider> {
    fn create(ind_provider: IndicatorProvider) -> T;
}
