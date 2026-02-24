use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(glam::Vec3);

impl Vec3 {
    pub const ZERO: Self = Self(glam::Vec3::ZERO);
    pub const ONE: Self = Self(glam::Vec3::ONE);
    pub const X: Self = Self(glam::Vec3::X);
    pub const Y: Self = Self(glam::Vec3::Y);
    pub const Z: Self = Self(glam::Vec3::Z);

    pub fn len(&self) -> f32 {
        self.0.length()
    }

    pub fn len_sqr(&self) -> f32 {
        self.0.length_squared()
    }

    pub fn horiz_len_sqr(&self) -> f32 {
        self.x() * self.x() + self.z() * self.z()
    }

    pub fn horiz_len(&self) -> f32 {
        self.horiz_len_sqr().sqrt()
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(glam::Vec3::new(x, y, z))
    }

    pub fn splat(a: f32) -> Self {
        Self(glam::Vec3::splat(a))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn z(&self) -> f32 {
        self.0.z
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0.x
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0.y
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.0.z
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Add<(f32, f32, f32)> for Vec3 {
    type Output = Self;

    fn add(self, rhs: (f32, f32, f32)) -> Self {
        Self(self.0 + glam::Vec3::from(rhs))
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<(f32, f32, f32)> for Vec3 {
    fn add_assign(&mut self, rhs: (f32, f32, f32)) {
        self.0 += glam::Vec3::from(rhs)
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Sub<(f32, f32, f32)> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: (f32, f32, f32)) -> Self {
        Self(self.0 - glam::Vec3::from(rhs))
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl Mul<(f32, f32, f32)> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: (f32, f32, f32)) -> Self {
        Self(self.0 * glam::Vec3::from(rhs))
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

impl MulAssign<(f32, f32, f32)> for Vec3 {
    fn mul_assign(&mut self, rhs: (f32, f32, f32)) {
        self.0 *= glam::Vec3::from(rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(v: (f32, f32, f32)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}