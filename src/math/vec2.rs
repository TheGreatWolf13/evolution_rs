use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec2(glam::Vec2);

impl Vec2 {
    pub const ZERO: Self = Self(glam::Vec2::ZERO);
    pub const ONE: Self = Self(glam::Vec2::ONE);
    pub const X: Self = Self(glam::Vec2::X);
    pub const Y: Self = Self(glam::Vec2::Y);

    pub fn new(x: f32, y: f32) -> Self {
        Self(glam::Vec2::new(x, y))
    }

    pub fn splat(v: f32) -> Self {
        Self(glam::Vec2::splat(v))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0.x
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0.y
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Self::new(v.0, v.1)
    }
}