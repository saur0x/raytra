use crate::{
    color::Color,
    vector::Vector,
    hitrecord::HitRecord,
    ray::Ray,
    scene::Scene,
    utils
};


pub trait Material {
    fn scatter(&self, hit_record: &HitRecord) -> Ray;
    // fn color_at(&self, hit_record: &HitRecord) -> Color;
    fn color_at(&self, hit_record: &HitRecord, scene: &Scene) -> Color;

    fn get_attenuation(&self, hit_record: &HitRecord) -> Color;
    fn get_ambience(&self) -> f64;
    fn get_diffuse(&self) -> f64;
    fn get_specular(&self) -> f64;
    fn get_reflection(&self) -> f64;
}


pub struct Dielectric {
    color: Color,
    refractive_index: f64,

    pub ambience: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64    
}

impl Dielectric {
    pub fn new(color: Color, refractive_index: f64, ambience: f64, diffuse: f64, specular: f64, reflection: f64) -> Self {
        Self { color, refractive_index, ambience, diffuse, specular, reflection }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r = (1.0 - ref_idx) / (1.0 + ref_idx);
        r *= r;
        r + (1.0 - r) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &HitRecord) -> Ray {
        let refraction_ratio = if hit_record.is_out { 1.0 / self.refractive_index } else { self.refractive_index };
        let unit_direction = hit_record.ray.direction.normalize();
        let refracted = unit_direction.refract(hit_record.normal, refraction_ratio);
        // return Ray::new(hit_record.position, refracted);

        let cos_theta = f64::min(hit_record.normal.dot(-unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > utils::random_double(0.0, 1.0) {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };

        Ray::new(hit_record.position, direction)
    }

    fn color_at(&self, hit_record: &HitRecord, scene: &Scene) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn get_attenuation(&self, hit_record: &HitRecord) -> Color {
        // Color::from_hex(0xFFFFFF)
        // self.color
        Color::new(1.0, 1.0, 1.0)
    }

    fn get_ambience(&self) -> f64 { self.ambience }
    fn get_diffuse(&self) -> f64 { self.diffuse }
    fn get_specular(&self) -> f64 { self.specular }
    fn get_reflection(&self) -> f64 { self.reflection }
}


pub struct Metal {
    color: Color,

    pub ambience: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            color: Color::from_hex(0xFFFFFF),
            ambience: 0.05,
            diffuse: 1.0,
            specular: 1.0,
            reflection: 0.2
        }
    }
}

impl Metal {
    pub fn new(color: Color, ambience: f64, diffuse: f64, specular: f64, reflection: f64) -> Self {
        Self {
            color, ambience, diffuse, specular, reflection
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            color, ..Self::default()
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord) -> Ray {
        Ray::new(
            hit_record.position + hit_record.normal * 0.0001,
            hit_record.ray.direction.reflect(hit_record.normal)
        )
    }

    fn color_at(&self, hit_record: &HitRecord, scene: &Scene) -> Color {
        let hit_position = hit_record.position;
        let hit_normal = hit_record.normal;
        // let material = hit_record.object.get_material();

        let object_color = self.color;
        let hit_to_camera = scene.camera - hit_position;
        let specular_k = 50.0;

        // Ambience
        // let mut color = self.get_ambience() * Color::from_hex(0xFFFFFF);
        let mut color = self.get_ambience() * Color::from_hex(0x000000);

        for light in scene.lights.iter() {
            let hit_to_light = Ray::new(hit_position, light.position - hit_position);
            let half_vector = (hit_to_light.direction + hit_to_camera).normalize();

            // Diffuse (Lambert) Shading
            color += f64::max(hit_normal.dot(hit_to_light.direction), 0.0)
                * self.get_diffuse()
                * object_color;

            // Specular (Blinn-Phong) Shading
            color += f64::max(hit_normal.dot(half_vector), 0.0).powf(specular_k)
                * self.get_specular()
                * light.color;
        }
        color
    }

    fn get_attenuation(&self, hit_record: &HitRecord) -> Color {
        self.get_reflection() * Color::new(1.0, 1.0, 1.0)
        // Color::new(0.0, 0.0, 0.0)
        // Color::new(1.0, 1.0, 1.0)
    }

    fn get_ambience(&self) -> f64 { self.ambience }
    fn get_diffuse(&self) -> f64 { self.diffuse }
    fn get_specular(&self) -> f64 { self.specular }
    fn get_reflection(&self) -> f64 { self.reflection }
}


pub struct Checkered {
    color0: Color,
    color1: Color,
    dx: f64,
    dy: f64,
    dz: f64,

    pub ambience: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64
}

impl Default for Checkered {
    fn default() -> Self {
        Self {
            color0: Color::from_hex(0x420500),
            color1: Color::from_hex(0xE6B87D),
            dx: 4.0,
            dy: 4.0,
            dz: 4.0,
            ambience: 0.2,
            diffuse: 0.5,
            specular: 0.5,
            reflection: 0.2
        }
    }
}

impl Checkered {
    pub fn new(color0: Color, color1: Color, dx: f64, dy: f64, dz: f64, ambience: f64, diffuse: f64, specular: f64, reflection: f64) -> Self {
        Self {
            color0, color1, dx, dy, dz, ambience, diffuse, specular, reflection
        }
    }
}

impl Material for Checkered {
    fn scatter(&self, hit_record: &HitRecord) -> Ray {
        Ray::new(hit_record.position, hit_record.ray.direction.reflect(hit_record.normal))
    }

    fn color_at(&self, hit_record: &HitRecord, scene: &Scene) -> Color {
        let hit_position = hit_record.position;
        let hit_normal = hit_record.normal;
        // let material = hit_record.object.get_material();
        let x = (hit_record.position.0 / self.dx).ceil() as i32;
        let z = (hit_record.position.2 / self.dz).ceil() as i32;
        
        let object_color = if x & 1 == z & 1 {
            self.color0
        } else {
            self.color1
        };
        let hit_to_camera = scene.camera - hit_position;
        let specular_k = 50.0;

        // Ambience
        let mut color = self.get_ambience() * Color::from_hex(0xFFFFFF);

        for light in scene.lights.iter() {
            let hit_to_light = Ray::new(hit_position, light.position - hit_position);
            let half_vector = (hit_to_light.direction + hit_to_camera).normalize();

            // Diffuse (Lambert) Shading
            color += f64::max(hit_normal.dot(hit_to_light.direction), 0.0)
                * self.get_diffuse()
                * object_color;

            // Specular (Blinn-Phong) Shading
            color += f64::max(hit_normal.dot(half_vector), 0.0).powf(specular_k)
                * self.get_specular()
                * light.color;
        }
        color
    }

    fn get_attenuation(&self, hit_record: &HitRecord) -> Color {
        // Color::from_hex(0xFFFFFF)
        self.get_reflection() * Color::new(1.0, 1.0, 1.0)
    }

    fn get_ambience(&self) -> f64 { self.ambience }
    fn get_diffuse(&self) -> f64 { self.diffuse }
    fn get_specular(&self) -> f64 { self.specular }
    fn get_reflection(&self) -> f64 { self.reflection }
}