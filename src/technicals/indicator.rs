use super::serie::Serie;

pub trait Indicator<'a> {
    fn name() -> String where Self: Sized;
    fn series(&'a self) -> &'a Vec<Serie<'a>>;
    // color
}
