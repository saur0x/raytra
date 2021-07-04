use crate::{
    image::Image,
    ray::Ray,
    color::Color,
    point::Point,
    scene::Scene,
    object::{Object, Sphere},
    vector::Vector,
    HitRecord,
    Rc,
    utils
};


#[derive(Default)]
pub struct RenderEngine {
    max_depth: isize,
    min_displacement: f64,
    rays_per_pixel: usize
}

impl RenderEngine {
    pub fn new(max_depth: isize, rays_per_pixel: usize) -> Self {
        Self {
            max_depth, min_displacement: 0.001, rays_per_pixel
        }
    }
}

impl RenderEngine {
    /// Find the nearest object hit by the ray in the scene.
    fn find_nearest(&self, ray: Ray, scene: &Scene, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut min_record: Option<HitRecord> = None;
        let mut min_distance: f64 = utils::INFINITY;

        for object in scene.objects.to_vec() {
            let mut hit_record = HitRecord::new(Rc::clone(&object));
            let is_hit = object.hit(ray, t_min, t_max, &mut hit_record);
            if is_hit && (matches!(min_record, None) || hit_record.distance < min_distance) {
                min_distance = hit_record.distance;
                min_record = Some(hit_record);
                if min_distance <= 0.0 { eprintln!("ERROR: Negative distance to object."); }
            }
        }
        min_record
    }

    pub fn render(&self, scene: &mut Scene) -> Image {
        let width = scene.width;
        let height = scene.height;
        let camera = scene.camera;

        let aspect_ratio = width as f64 / height as f64;

        // (x0, y0) is bottom-left.
        let x0 = -1.0;
        let x1 =  1.0;
        let dx = (x1 - x0) / (width - 1) as f64;

        let y0 = -1.0 / aspect_ratio;
        let y1 =  1.0 / aspect_ratio;
        let dy = (y1 - y0) / (height - 1) as f64;

        let mut pixels = Image::new(scene.width, scene.height);

        let mut x: f64;
        let mut y: f64;

        for j in 0..height {
            // y = y0 + (height - j - 1) as f64 * dy;
            for i in 0..width {
                // x = x0 + i as f64 * dx;
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.rays_per_pixel {
                    y = y0 + (utils::random_double(0.0, 1.0) + (height - j - 1) as f64) * dy;
                    x = x0 + (utils::random_double(0.0, 1.0) + i as f64) * dx;
                    let ray = Ray::new(camera, Vector::new(x, y, -1.0) - camera);
                    color += self.ray_trace(ray, scene, self.max_depth);
                }
                pixels.set_pixel(i, j, (color / self.rays_per_pixel as f64).powf(0.5));
            }
            eprint!("\r> {} %", j * 100 / (height - 1));
        }
        eprintln!();
        pixels
    }

    fn ray_trace(&self, ray: Ray, scene: &Scene, depth: isize) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit_record = self.find_nearest(ray, scene, 0.0001, utils::INFINITY);

        if matches!(hit_record, None) {
            // return Color::new(0.0, 0.0, 0.0);
            let t = 0.5 * (ray.direction.normalize().1 + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }

        let hit_record = hit_record.unwrap();
        let material = hit_record.object.get_material();
        let scattered = material.scatter(&hit_record);
        let attenuation = material.get_attenuation(&hit_record);
        material.color_at(&hit_record, scene)
        + (self.ray_trace(scattered, scene, depth - 1) * attenuation)
    }
}
