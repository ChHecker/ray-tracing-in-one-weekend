use crate::vec3::Point;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Point,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
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

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Point {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
