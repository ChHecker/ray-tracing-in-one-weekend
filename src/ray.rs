//! A simple ray of light.

use nalgebra::Vector3;

/// A ray starting at `origin` at `time` pointing in `direction`.
///
/// # Fields
/// - `origin`: [Point] of emission.
/// - `direction`: direction.
/// - `time`: time.
#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
    time: f32,
}

impl Ray {
    /// Create a ray without a time.
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
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

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    /// Calculate the ray at parameter `t`.
    ///
    /// # Examples
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, ray::Ray};
    /// let origin = vector![1., 0., 0.];
    /// let direction = vector![0., 1., 0.];
    /// let ray = Ray::new(origin, direction);
    /// assert_eq!(ray.at(1.), vector![1., 1., 0.]);
    /// ```
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
