use crate::{
    point::Point,
    ray::Ray,
    color::Color,
    vector::Vector,
    material::Material,
    hitrecord::HitRecord,
    Rc,
    utils
};


pub trait Object {
    /// Update `hit_record` with details if there's a hit, else return false.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;

    /// Return a clone of `material` pointer.
    fn get_material(&self) -> Rc<dyn Material>;
}


pub struct Sphere {
    center: Point,
    radius: f64,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Take the least out of two roots.
        let b_a = -b / a;
        let d_a = discriminant_sqrt / a;
        let mut distance = b_a - d_a;
        if distance < t_min || distance >= t_max {
            distance = b_a + d_a;
            if distance < t_min || distance >= t_max {
                return false;
            }
        }

        // `normal` must always be in opposite direction of incident ray.
        let normal = (ray.origin + ray.direction * distance - self.center).normalize();
        let is_out = normal.dot(ray.direction) < 0.0;
        hit_record.update(
            ray,
            distance,
            if is_out { normal } else { -normal },
            is_out
        );
        true
    }

    fn get_material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
}


pub struct Plane {
    unit_normal: Vector,
    distance: f64,
    pub material: Rc<dyn Material>
}

impl Plane {
    pub fn new(unit_normal: Vector, distance: f64, material: Rc<dyn Material>) -> Self {
        Self { unit_normal: unit_normal.normalize(), distance, material }
    }
}

impl Object for Plane {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        // `ray` doesn't intersect if the ray direction is perpendicular to the plane normal.
        let dot = self.unit_normal.dot(ray.direction);
        
        if dot == 0.0 {
            return false;
        }

        let distance = (self.distance - 2.0 * self.unit_normal.dot(ray.origin)) / dot;

        if !(distance >= 0.0 && distance >= t_min && distance <= t_max) {
            return false;
        }

        // `hit_normal` must always be in opposite direction of incident ray.
        let normal = self.unit_normal;
        let is_out = self.unit_normal.dot(ray.direction) <= 0.0;
        hit_record.update(
            ray,
            distance,
            if is_out { normal } else { -normal },
            is_out
        );
        true
    }

    fn get_material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
}


pub struct Parallelepiped {
    x: Vector,
    y: Vector,
    z: Vector,
    origin: Vector,
    material: Rc<dyn Material>
}

impl Parallelepiped {
    pub fn new(x: Vector, y: Vector, z: Vector, origin: Vector, material: Rc<dyn Material>) -> Self {
        Self { x, y, z, origin, material }
    }
}

impl Object for Parallelepiped {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        // `intersections` contains distance of intersection for each face of the solid.
        let intersections: Vec<Option<f64>> = vec![
            Parallelogram::new(self.origin, self.x, self.y)
                .intersects(&ray),
            Parallelogram::new(self.origin + self.z, self.x, self.y)
                .intersects(&ray),
            Parallelogram::new(self.origin, self.y, self.z)
                .intersects(&ray),
            Parallelogram::new(self.origin + self.x, self.y, self.z)
                .intersects(&ray),
            Parallelogram::new(self.origin, self.x, self.z)
                .intersects(&ray),
            Parallelogram::new(self.origin + self.y, self.x, self.z)
                .intersects(&ray),
        ];

        let mut min_index: usize = 100;
        let mut min_distance: f64 = utils::INFINITY;

        // Find the face that has closest hit.
        for (i, distance) in intersections.iter().enumerate() {
            if matches!(distance, Some(d) if d >= &t_min && d < &t_max && d < &min_distance) {
                min_distance = distance.unwrap();
                min_index = i;
            }
        }

        // Return false if no hit.
        if min_index >= 6 {
            return false;
        }

        // Outward normal is different for each face.
        let normal = match min_index {
            0 => self.y.cross(self.x),
            1 => self.x.cross(self.y),
            2 => self.z.cross(self.y),
            3 => self.y.cross(self.z),
            4 => self.x.cross(self.z),
            5 => self.z.cross(self.x),
            _ => return false
        }.normalize();

        let is_out = normal.dot(ray.direction) <= 0.0;
        hit_record.update(
            ray,
            intersections[min_index].unwrap(),
            if is_out { normal } else { -normal },
            is_out
        );
        true
    }

    fn get_material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }
}


struct Parallelogram {
    origin: Vector,
    x: Vector,
    y: Vector
}

impl Parallelogram {
    pub fn new(origin: Vector, x: Vector, y: Vector) -> Self {
        Self { origin, x, y }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<f64> {
        let x_position = self.origin + self.x;
        let y_position = self.origin + self.y;

        let cross = (x_position - self.origin).cross(y_position - self.origin);
        let dot = cross.dot(ray.direction);
        if dot == 0.0 {
            return None;
        }

        let distance = (cross.dot(self.origin) - cross.dot(ray.origin)) / dot;
        if distance < 0.0 || !self.is_inside(ray.origin + ray.direction * distance) {
            None
        } else {
            Some(distance)
        }
    }

    fn is_inside(&self, point: Vector) -> bool {
        const DELTA: f64 = 1E-5;
        let volume = self.x.cross(self.y).magnitude();
        let calculated: f64 = ((point - self.origin).cross(self.x).magnitude()
            + (point - self.origin).cross(self.y).magnitude()
            + (point - self.origin - self.x).cross(self.y).magnitude()
            + (point - self.origin - self.y).cross(self.x).magnitude()
        ) / 2.0;
        (calculated - volume).abs() < DELTA
    }
}