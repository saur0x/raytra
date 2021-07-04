use crate::point::Point;
use crate::color::Color;


/// Light represents a point light source of a certain color.
pub struct Light {
    pub position: Point,
    pub color: Color
}

impl Light {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}
