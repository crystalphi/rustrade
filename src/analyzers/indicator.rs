use super::serie::Serie;

pub trait Indicator<'a> {
    fn name(&self) -> String;
    fn series(&'a self) -> &'a Vec<Serie<'a>>;
    // color
}
