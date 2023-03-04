use crate::vec3::Point3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time;
        self
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
