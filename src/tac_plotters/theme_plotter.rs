use plotters::style::{RGBColor, BLACK, WHITE};
pub struct ThemePlotter {}

impl ThemePlotter {
    pub fn back_ground() -> RGBColor {
        WHITE
    }

    pub fn fore_ground() -> RGBColor {
        BLACK
    }
}
