#![allow(dead_code, unused_imports, unused_variables)]

extern crate rand;

mod vector;
mod color;
mod point;
mod image;
mod utils;
mod object;
mod ray;
mod scene;
mod engine;
mod light;
mod material;
mod hitrecord;

use vector::Vector;
use color::Color;
use point::Point;
use scene::Scene;
use object::{Object, Sphere, Plane, Parallelepiped};
use engine::RenderEngine;
use light::Light;
use material::{Material, Metal, Checkered, Dielectric};
use hitrecord::HitRecord;
use std::rc::Rc;


// 4K
// const WIDTH: usize = 3840;
// const HEIGHT: usize = 2160;

// 1080
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;


fn main() {
    let camera = Vector::new(0.0, 0.0, 0.0);

    let lights = vec![
        // Light::new(Point::new(0.0, 4.0, 4.0), Color::from_hex(0xFFFFFF)),
        // Light::new(Point::new(0.0, 0.0, 0.0), Color::from_hex(0xFFFFFF))
        Light::new(Point::new(0.0, 3.0, 2.0), Color::from_hex(0xFFFFFF))
    ];

    let objects: Vec<Rc<dyn Object>> = vec![
        Rc::new(
            // Sphere::new(Point::new(-1.2, 0.5, -2.0), 0.5,
            Sphere::new(Point::new(-1.1, 0.0, -1.8), 0.5,
                Rc::new(
                    Metal::new(Color::from_hex(0x314e52), 0.05, 1.0, 0.5, 0.05)
                )
            )
        ),

        Rc::new(
            Sphere::new(Point::new(0.0, 0.1, -1.0), 0.4,
            // Parallelepiped::new(Vector::new(0.3535, 0.3535, 0.0), Vector::new(-0.3535, 0.3535, 0.0), Vector::new(0.0, 0.3535, -0.3535), Vector::new(0.0, -0.4, -1.0),
            // Parallelepiped::new(Vector::new(0.5, 0.0, 0.0), Vector::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, -0.5), Vector::new(-0.25, -0.25, -1.0),

                Rc::new(
                    // Metal::new(Color::from_hex(0xeb5e0b), 0.05, 0.5, 0.1, 0.5)
                    Dielectric::new(Color::from_hex(0xFFFFFF), 2.42, 0.0, 0.0, 0.0, 0.0)
                )
            )
        ),

        Rc::new(
            Sphere::new(Point::new(1.1, 0.0, -2.0), 0.5,
                Rc::new(
                    Metal::new(Color::from_hex(0xffd384), 0.05, 0.2, 0.2, 0.8)
                )
            )
        ),

        Rc::new(
            // Plane::new(Vector::new(0.0, -1.0, 0.1), 0.5,
            Plane::new(Vector::new(0.0, -1.0, 0.0), 0.5,
                Rc::new(
                    Checkered::new(Color::from_hex(0xEED6D3), Color::from_hex(0x67595E), 0.25, 0.25, 0.25, 0.0, 1.0, 0.2, 0.2)
                )
            )
        )
    ];

    let mut scene = Scene::new(camera, objects, lights, WIDTH, HEIGHT);

    // TODO: Add a `Camera` type and pass scene and camera to `RenderEngine::new`.
    let engine = RenderEngine::new(10, 5);
    let image = engine.render(&mut scene);
    image.show();
}
