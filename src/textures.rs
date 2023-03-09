//! A way to apply textures to shapes.

use std::fmt::Debug;
use std::sync::Arc;

use crate::*;

/// An abstraction over all textures.
///
/// `Send + Sync` is necessary for multithreading.
pub trait Texture: Debug + Send + Sync {
    /// Calculate the color of the texture.
    ///
    /// # Parameters:
    /// - (`u`, `v`): Coordinates on the surface submanifold (lie inside \[0,1\]).
    /// - `hit_point`: [Point] where the [`ray::Ray`] hit the texture.
    fn color_at(&self, u: f32, v: f32, hit_point: Point) -> Color;
}

/// A solid color texture.
#[derive(Debug)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn color_at(&self, _u: f32, _v: f32, _hit_point: Point) -> Color {
        self.color
    }
}

/// A checkerboard texture.
#[derive(Debug)]
pub struct CheckerTexture<S: Texture, T: Texture> {
    texture_even: Arc<S>,
    texture_odd: Arc<T>,
}

impl<S: Texture, T: Texture> CheckerTexture<S, T> {
    pub fn new(texture_even: Arc<S>, texture_odd: Arc<T>) -> Self {
        Self {
            texture_even,
            texture_odd,
        }
    }
}

impl<S: Texture, T: Texture> Texture for CheckerTexture<S, T> {
    fn color_at(&self, u: f32, v: f32, hit_point: Point) -> Color {
        let sin_product =
            (10. * hit_point.x()).sin() * (10. * hit_point.y()).sin() * (10. * hit_point.z()).sin();
        if sin_product < 0. {
            self.texture_odd.color_at(u, v, hit_point)
        } else {
            self.texture_even.color_at(u, v, hit_point)
        }
    }
}
