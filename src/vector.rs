use std::ops;


#[derive(Debug, Copy, Clone)]
pub struct Vector(pub f64, pub f64, pub f64);


/// Instance methods
impl Vector {
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            (self.1 * other.2) - (other.1 * self.2),
            (self.2 * other.0) - (other.2 * self.0),
            (self.0 * other.1) - (other.0 * self.1)
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude)
    }

    pub fn update(&mut self, x: f64, y: f64, z: f64) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
    }

    pub fn update_from_vector(&mut self, other: Self) {
        self.update(other.0, other.1, other.2);
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }
    
    pub fn refract(&self, normal: Self, refraction_ratio: f64) -> Self {
        let cos_theta = f64::min(normal.dot(-1.0 * (*self)), 1.0);
        // let cos_theta = f64::min(self.dot(-normal), 1.0);
        let r_perpendicular: Self = refraction_ratio * (*self + cos_theta * normal);
        // let r_out_parallel = -(1.0 - r_out_perp.dot(r_out_perp)).abs().sqrt() * normal;
        let r_parallel = -1.0 * (1.0 - r_perpendicular.magnitude().powi(2)).abs().sqrt() * normal;
        r_perpendicular + r_parallel
    }

    pub fn powf(&self, pow: f64) -> Self {
        if pow == 0.5 {
            Self(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
        } else {
            Self(self.0.powf(pow), self.1.powf(pow), self.2.powf(pow))
        }
    }

    pub fn is_small(&self) -> bool {
        const DELTA: f64 = 1E-3;
        self.0.abs() < DELTA && self.1.abs() < DELTA && self.2.abs() < DELTA
    }
}

/// Struct methods
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn dot_product(u: Self, v: Self) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }
    
    pub fn cross_product(u: Self, v: Self) -> Vector {
        Self(
            (u.1 * v.2) - (v.1 * u.2),
            (u.2 * v.0) - (v.2 * u.0),
            (u.0 * v.1) - (v.0 * u.1)
        )
    }
}


// Default trait and operator overloading. Nothing special below this line.

impl Default for Vector {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }    
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }    
}

impl ops::Mul<Vector> for Vector {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, other: Self::Output) -> Self::Output {
        Vector(self * other.0, self * other.1, self * other.2)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl ops::Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("IndexError")
        }
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("IndexError")
        }
    }
}
