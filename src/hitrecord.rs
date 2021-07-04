use crate::object::Object;
use crate::vector::Vector;
use crate::ray::Ray;
use crate::point::Point;
use crate::Rc;


pub struct HitRecord {
    pub object: Rc<dyn Object>,
    pub ray: Ray,
    pub distance: f64,

    pub normal: Vector,
    pub position: Point,
    pub is_out: bool
}

impl HitRecord {
    pub fn new(object: Rc<dyn Object>) -> Self {
        Self {
            object,
            ray: Ray::default(),
            distance: f64::default(),
            normal: Vector::default(),
            position: Point::default(),
            is_out: bool::default()
        }
    }

    pub fn update_object(&mut self, object: Rc<dyn Object>) {
        self.object = object;
    }

    pub fn update(&mut self, ray: Ray, distance: f64, normal: Vector, is_out: bool) {
        self.ray = ray;
        self.distance = distance;
        self.normal = normal;
        self.position = ray.origin + ray.direction * distance;
        self.is_out = is_out;
    }
}