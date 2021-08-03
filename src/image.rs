use std::io::Write;

use crate::color::Color;
use crate::utils;


#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}


impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height, pixels: vec![Color::default(); height * width]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        self.pixels[y * self.width + x] = pixel;
    }

    pub fn show(&self) {
        let mut pixels = Vec::<u8>::with_capacity(3 * self.width * self.height);
        for c in self.pixels.iter() {
            pixels.extend_from_slice(&[Self::to_byte(c.0), Self::to_byte(c.1), Self::to_byte(c.2)]);
        }

        print!("P6\n{} {}\n255\n", self.width, self.height);
        std::io::stdout().write_all(&pixels).expect("Error occurred while writing pixel data.");
    }

    #[inline]
    fn to_byte(value: f64) -> u8 {
        (256.0 * utils::clamp(value, 0.0, 0.999)) as u8
    }
}
