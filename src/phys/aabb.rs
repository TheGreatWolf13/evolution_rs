use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct AABB {
    epsilon: f32,
    pub start: Vec3,
    pub end: Vec3,
}

impl AABB {
    pub fn new(start: impl Into<Vec3>, end: impl Into<Vec3>) -> AABB {
        let start = start.into();
        let end = end.into();
        AABB {
            epsilon: 0.0,
            start,
            end,
        }
    }

    pub fn expand(&self, a: impl Into<Vec3>) -> AABB {
        let a = a.into();
        let mut start = self.start;
        let mut end = self.end;
        if a.x() < 0.0 {
            *start.x_mut() += a.x();
        } //
        else if a.x() > 0.0 {
            *end.x_mut() += a.x();
        }
        if a.y() < 0.0 {
            *start.y_mut() += a.y();
        } //
        else if a.y() > 0.0 {
            *end.y_mut() += a.y();
        }
        if a.z() < 0.0 {
            *start.z_mut() += a.z();
        } //
        else if a.z() > 0.0 {
            *end.z_mut() += a.z();
        }
        AABB::new(start, end)
    }

    pub fn grow(&self, a: impl Into<Vec3>) -> AABB {
        let a = a.into();
        AABB::new(self.start - a, self.end + a)
    }

    pub fn clip_x_collide(&self, c: &AABB, xa: f32) -> f32 {
        if c.end.y() <= self.start.y() || c.start.y() >= self.end.y() {
            return xa;
        }
        if c.end.z() <= self.start.z() || c.start.z() >= self.end.z() {
            return xa;
        }
        let mut xa = xa;
        let max = self.start.x() - c.end.x() - self.epsilon;
        if xa > 0.0 && c.end.x() <= self.start.x() && max < xa {
            xa = max;
        }
        let max = self.end.x() - c.start.x() + self.epsilon;
        if xa < 0.0 && c.start.x() >= self.end.x() && max > xa {
            xa = max;
        }
        xa
    }

    pub fn clip_y_collide(&self, c: &AABB, ya: f32) -> f32 {
        if c.end.x() <= self.start.x() || c.start.x() >= self.end.x() {
            return ya;
        }
        if c.end.z() <= self.start.z() || c.start.z() >= self.end.z() {
            return ya;
        }
        let mut ya = ya;
        let max = self.start.y() - c.end.y() - self.epsilon;
        if ya > 0.0 && c.end.y() <= self.start.y() && max < ya {
            ya = max;
        }
        let max = self.end.y() - c.start.y() + self.epsilon;
        if ya < 0.0 && c.start.y() >= self.end.y() && max > ya {
            ya = max;
        }
        ya
    }

    pub fn clip_z_collide(&self, c: &AABB, za: f32) -> f32 {
        if c.end.x() <= self.start.x() || c.start.x() >= self.end.x() {
            return za;
        }
        if c.end.y() <= self.start.y() || c.start.y() >= self.end.y() {
            return za;
        }
        let mut za = za;
        let max = self.start.z() - c.end.z() - self.epsilon;
        if za > 0.0 && c.end.z() <= self.start.z() && max < za {
            za = max;
        }
        let max = self.end.z() - c.start.z() + self.epsilon;
        if za < 0.0 && c.start.z() >= self.end.z() && max > za {
            za = max;
        }
        za
    }

    pub fn r#move(&mut self, a: impl Into<Vec3>) {
        let a = a.into();
        self.start += a;
        self.end += a;
    }
}