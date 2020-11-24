use plotters::prelude::*;
use plotters::{
    coord::Shift,
    prelude::{BitMapBackend, DrawingArea},
};

use crate::technicals::indicator::Indicator;

pub fn indicator_plotter<'a, T>(indicator: T, root: &DrawingArea<BitMapBackend, Shift>)
where
    T: Indicator<'a>,
{
    //
}
