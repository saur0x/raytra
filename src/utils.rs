use rand::Rng;


pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;


pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    f64::max(f64::min(x, max), min)
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}