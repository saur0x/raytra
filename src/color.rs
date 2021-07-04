pub type Color = crate::vector::Vector;


impl Color {
    pub fn from_hex(hex: u32) -> Self {
        Self(
            (hex >> 16 & 0xFF) as f64 / 255.0,
            (hex >> 8  & 0xFF) as f64 / 255.0,
            (hex       & 0xFF) as f64 / 255.0
        )
    }
}