use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign};

pub trait Angle {
    fn to_degrees(&self) -> AngleDeg;

    fn to_radians(&self) -> AngleRad;

    fn to_revolutions(&self) -> AngleRev;

    fn sin(&self) -> f32;

    fn cos(&self) -> f32;

    fn sin_cos(&self) -> (f32, f32);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleDeg(f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRad(f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRev(f32);

impl Angle for AngleDeg {
    fn to_degrees(&self) -> AngleDeg {
        *self
    }

    fn to_radians(&self) -> AngleRad {
        AngleRad(self.0.to_radians())
    }

    fn to_revolutions(&self) -> AngleRev {
        AngleRev(self.0 / 360.0)
    }

    fn sin(&self) -> f32 {
        self.0.to_radians().sin()
    }

    fn cos(&self) -> f32 {
        self.0.to_radians().cos()
    }

    fn sin_cos(&self) -> (f32, f32) {
        let rad = self.0.to_radians();
        (rad.sin(), rad.cos())
    }
}

impl Angle for AngleRad {
    fn to_degrees(&self) -> AngleDeg {
        AngleDeg(self.0.to_degrees())
    }

    fn to_radians(&self) -> AngleRad {
        *self
    }

    fn to_revolutions(&self) -> AngleRev {
        AngleRev(self.0 / (2.0 * PI))
    }

    fn sin(&self) -> f32 {
        self.0.sin()
    }

    fn cos(&self) -> f32 {
        self.0.cos()
    }

    fn sin_cos(&self) -> (f32, f32) {
        (self.0.sin(), self.0.cos())
    }
}

impl Angle for AngleRev {
    fn to_degrees(&self) -> AngleDeg {
        AngleDeg(self.0 * 360.0)
    }

    fn to_radians(&self) -> AngleRad {
        AngleRad(self.0 * (2.0 * PI))
    }

    fn to_revolutions(&self) -> AngleRev {
        *self
    }

    fn sin(&self) -> f32 {
        (self.0 * (2.0 * PI)).sin()
    }

    fn cos(&self) -> f32 {
        (self.0 * (2.0 * PI)).cos()
    }

    fn sin_cos(&self) -> (f32, f32) {
        let rad = self.0 * (2.0 * PI);
        (rad.sin(), rad.cos())
    }
}

impl Deref for AngleDeg {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AngleRad {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AngleRev {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AngleDeg {
    pub const ZERO: AngleDeg = Self::new(0.0);

    pub const fn new(deg: f32) -> Self {
        Self(deg)
    }
}

impl AngleRad {
    pub const ZERO: AngleRad = AngleRad(0.0);

    pub fn new(radians: f32) -> Self {
        Self(radians)
    }
}

impl AngleRev {
    pub const ZERO: AngleRev = Self::new(0.0);

    pub const fn new(revolutions: f32) -> Self {
        Self(revolutions)
    }
}

impl Add<AngleDeg> for AngleDeg {
    type Output = AngleDeg;

    fn add(self, rhs: AngleDeg) -> Self::Output {
        AngleDeg(self.0 + rhs.0)
    }
}

impl AddAssign<AngleDeg> for AngleDeg {
    fn add_assign(&mut self, rhs: AngleDeg) {
        self.0 += rhs.0;
    }
}

impl Mul<f32> for AngleDeg {
    type Output = AngleDeg;

    fn mul(self, rhs: f32) -> Self::Output {
        AngleDeg(self.0 * rhs)
    }
}

impl MulAssign<f32> for AngleDeg {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rot3Deg(AngleDeg, AngleDeg, AngleDeg);

impl Rot3Deg {
    pub const ZERO: Rot3Deg = Self::new(AngleDeg::ZERO, AngleDeg::ZERO, AngleDeg::ZERO);

    pub const fn new(x: AngleDeg, y: AngleDeg, z: AngleDeg) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> AngleDeg {
        self.0
    }

    pub fn y(&self) -> AngleDeg {
        self.1
    }

    pub fn z(&self) -> AngleDeg {
        self.2
    }

    pub fn x_mut(&mut self) -> &mut AngleDeg {
        &mut self.0
    }

    pub fn y_mut(&mut self) -> &mut AngleDeg {
        &mut self.1
    }

    pub fn z_mut(&mut self) -> &mut AngleDeg {
        &mut self.2
    }
}

impl Add<Rot3Deg> for Rot3Deg {
    type Output = Rot3Deg;

    fn add(self, rhs: Rot3Deg) -> Self::Output {
        Rot3Deg(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<Rot3Deg> for Rot3Deg {
    fn add_assign(&mut self, rhs: Rot3Deg) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul<f32> for Rot3Deg {
    type Output = Rot3Deg;

    fn mul(self, rhs: f32) -> Self::Output {
        Rot3Deg(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for Rot3Deg {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}