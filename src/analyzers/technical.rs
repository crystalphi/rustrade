use super::indicator::Indicator;

pub trait Technical<'a, T: Indicator<'a>> {
    fn indicators() -> Vec<T>;
}
