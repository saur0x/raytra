use crate::{
    vector::Vector,
    object::Object,
    light::Light,
    Rc
};


/// Scene has all the information needed for the ray-tracing engine.
pub struct Scene {
    pub camera: Vector,
    pub objects: Vec<Rc<dyn Object>>,
    pub lights: Vec<Light>,
    pub width: usize,
    pub height: usize
}


impl Scene {
    pub fn new(camera: Vector, objects: Vec<Rc<dyn Object>>, lights: Vec<Light>, width: usize, height: usize) -> Self {
        Self {
            camera, objects, lights, width, height
        }
    }
}

pub trait SceneAdd<T> {
    /// Add an object or light to the scene.
    fn add(&mut self, node: T);
}

impl SceneAdd<Rc<dyn Object>> for Scene {
    fn add(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
}

impl SceneAdd<Light> for Scene {
    fn add(&mut self, light: Light) {
        self.lights.push(light);
    }
}
