//! A way to apply textures to shapes.

use std::fmt::Debug;
use std::path::Path;

use image::io::Reader as ImageReader;
use image::{ImageError, RgbImage};

use crate::color::WHITE;
use crate::perlin::Perlin;
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct CheckerTexture<S: Texture, T: Texture> {
    texture_even: S,
    texture_odd: T,
}

impl<S: Texture, T: Texture> CheckerTexture<S, T> {
    pub fn new(texture_even: S, texture_odd: T) -> Self {
        Self {
            texture_even,
            texture_odd,
        }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn solid_colors(color_even: Color, color_odd: Color) -> Self {
        let texture_even = SolidColor::new(color_even);
        let texture_odd = SolidColor::new(color_odd);
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

/// A grayscale Perlin noise texture.
///
/// # Fields
/// - `noise`: Stores the [`Perlin`] object. This is generated automatically.
/// - `scale`: By how much the `hit_point` should be scaled.
#[derive(Clone, Debug)]
pub struct PerlinNoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl PerlinNoiseTexture {
    pub fn new(scale: f32) -> Self {
        let noise = Perlin::new();
        Self { noise, scale }
    }
}

impl Texture for PerlinNoiseTexture {
    fn color_at(&self, _u: f32, _v: f32, hit_point: Point) -> Color {
        WHITE
            * 0.5
            * (1. + (self.scale * hit_point.z() + 10. * self.noise.turbulance(hit_point, 7)).sin())
    }
}

/// A image texture.
#[derive(Clone, Debug)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Self {
        Self { image }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let image: RgbImage = ImageReader::open(path)?.decode()?.into_rgb8();
        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn color_at(&self, u: f32, v: f32, _hit_point: Point) -> Color {
        let mut i = (u.clamp(0., 1.) * self.image.width() as f32) as u32;
        let mut j = ((1. - v.clamp(0., 1.)) * self.image.height() as f32) as u32;
        if i >= self.image.width() {
            i = self.image.width() - 1;
        }
        if j >= self.image.height() {
            j = self.image.height() - 1;
        }

        self.image.get_pixel(i, j).clone().into()
    }
}
