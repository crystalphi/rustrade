use crate::analyzers::{indicator::Indicator, technical::Technical};
use plotters::prelude::*;
use plotters::{
    coord::Shift,
    prelude::{BitMapBackend, DrawingArea},
};

pub fn indicator_plotter<'a, T: Indicator<'a>>(
    indicator: T,
    root: &DrawingArea<BitMapBackend, Shift>,
) {
    //
}
