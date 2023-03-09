//! A simple ray of light.

use crate::vec3::Point;

/// A ray starting at `origin` at `time` pointing in `direction`.
///
/// # Fields
/// - `origin`: [Point] of emission.
/// - `direction`: direction.
/// - `time`: time.
#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Point,
    time: f32,
}

impl Ray {
    /// Create a ray without a time.
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    /// Consume `self` and creates a [Ray] with a time.
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

    /// Calculate the ray at parameter `t`.
    ///
    /// # Examples
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let origin = point![1., 0., 0.];
    /// let direction = point![0., 1., 0.];
    /// let ray = Ray::new(origin, direction);
    /// assert_eq!(ray.at(1.), point![1., 1., 0.]);
    /// ```
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
