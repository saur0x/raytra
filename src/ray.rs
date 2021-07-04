use crate::{
    point::Point,
    vector::Vector
};

/// Ray is a half-line with origin and a normalized direction.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 0.0)
        }
    }
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction: direction.normalize()
        }
    }
}