use std::ops;

use image::Rgb;

use crate::*;

/// Macro for [`Color::new`]
#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr $(,)*) => {
        Color::new($r, $g, $b)
    };
}

/// RGB color.
///
/// The colors a stored in RGB with each value between 0 and 1.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    /// Formats the [`Color`] as a [`String`], converting the `f32` RGB values to `u8`.
    pub(crate) fn to_color_str(&self) -> String {
        format!(
            "{} {} {}",
            (256. * self.r().clamp(0., 0.999)) as u8,
            (256. * self.g().clamp(0., 0.999)) as u8,
            (256. * self.b().clamp(0., 0.999)) as u8
        )
    }

    /// Converts to an [`u8`] array.
    pub(crate) fn to_rgb_array(&self) -> [u8; 3] {
        [
            (256. * self.r().clamp(0., 0.999)) as u8,
            (256. * self.g().clamp(0., 0.999)) as u8,
            (256. * self.b().clamp(0., 0.999)) as u8,
        ]
    }
}

impl Vec3 for Color {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Color(r, g, b)
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Rgb<u8> {
        Rgb([
            (256. * color.r().clamp(0., 0.999)) as u8,
            (256. * color.g().clamp(0., 0.999)) as u8,
            (256. * color.b().clamp(0., 0.999)) as u8,
        ])
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = rhs * *self;
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(rhs.0 * self.0, rhs.1 * self.1, rhs.2 * self.2)
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Color) {
        *self = rhs * *self;
    }
}

impl ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Color(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color(-self.0, -self.1, -self.2)
    }
}

impl ops::Index<u8> for Color {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bound"),
        }
    }
}

/// Iterator over the RGB values of [`Color`].
pub struct Color3Iter {
    index: usize,
    col3: Color,
}

impl IntoIterator for Color {
    type Item = f32;
    type IntoIter = Color3Iter;

    fn into_iter(self) -> Self::IntoIter {
        Color3Iter {
            index: 0,
            col3: self,
        }
    }
}

impl Iterator for Color3Iter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.col3.r(),
            1 => self.col3.g(),
            2 => self.col3.b(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl FromIterator<f32> for Color {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }
}

pub const WHITE: Color = Color(1., 1., 1.);
pub const BLACK: Color = Color(0., 0., 0.);
pub const GRAY: Color = Color(0.5, 0.5, 0.5);
pub const GREY: Color = Color(0.5, 0.5, 0.5);
pub const RED: Color = Color(1., 0., 0.);
pub const GREEN: Color = Color(0., 1., 0.);
pub const BLUE: Color = Color(0., 0., 1.);
