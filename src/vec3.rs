//! A simple 3D vector.

use std::fmt;

use rand::Rng;

/// A three-dimensional vector.
pub trait Vec3: Copy + Clone + fmt::Debug + PartialEq {
    fn new(x: f32, y: f32, z: f32) -> Self;

    /// Creates a random vector with each element between 0 and 1.
    fn random() -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
    }

    /// Creates a random vector with each element in a range.
    fn random_in_range(min: f32, max: f32) -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self::new(
            min + rng.gen::<f32>() * (max - min),
            min + rng.gen::<f32>() * (max - min),
            min + rng.gen::<f32>() * (max - min),
        )
    }
}
