use crate::vec3::Point3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Point3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
