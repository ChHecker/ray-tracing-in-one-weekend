use std::ops;

use image::Rgb;
use rand::Rng;

use crate::*;

/// Macro for [`Color::new`]
#[macro_export]
macro_rules! color {
    [$r:expr, $g:expr, $b:expr $(,)*] => {
        Color::new($r, $g, $b)
    };
}

/// RGB color.
///
/// The colors a stored in RGB with each value between 0 and 1.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    /// Creates a random vector with each element between 0 and 1.
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }

    /// Creates a random vector with each element in a range.
    pub fn random_in_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Color::new(
            min + rng.gen::<f32>() * (max - min),
            min + rng.gen::<f32>() * (max - min),
            min + rng.gen::<f32>() * (max - min),
        )
    }

    /// Formats the [`Color`] as a [`String`], converting the `f32` RGB values to `u8`.
    pub(crate) fn to_color_str(self) -> String {
        let rgb: [u8; 3] = self.into();
        format!("{} {} {}", rgb[0], rgb[1], rgb[2])
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        [
            (256. * color.r().clamp(0., 0.999)) as u8,
            (256. * color.g().clamp(0., 0.999)) as u8,
            (256. * color.b().clamp(0., 0.999)) as u8,
        ]
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Rgb<u8> {
        Rgb(color.into())
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        color![
            value[0] as f32 / 255.,
            value[1] as f32 / 255.,
            value[2] as f32 / 255.,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
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
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
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
        Color::new(rhs * self.r, rhs * self.g, rhs * self.b)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
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
        Color::new(rhs.r * self.r, rhs.g * self.g, rhs.b * self.b)
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
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
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
        Color::new(-self.r, -self.g, -self.b)
    }
}

impl ops::Index<u8> for Color {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bound"),
        }
    }
}

/// Iterator over the RGB values of [`Color`].
pub struct ColorIter {
    index: usize,
    col3: Color,
}

impl IntoIterator for Color {
    type Item = f32;
    type IntoIter = ColorIter;

    fn into_iter(self) -> Self::IntoIter {
        ColorIter {
            index: 0,
            col3: self,
        }
    }
}

impl Iterator for ColorIter {
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
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
        }
    }
}

pub const WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
};
pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const GRAY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
};
pub const GREY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
};
pub const RED: Color = Color {
    r: 1.,
    g: 0.,
    b: 0.,
};
pub const GREEN: Color = Color {
    r: 0.,
    g: 1.,
    b: 0.,
};
pub const BLUE: Color = Color {
    r: 0.,
    g: 0.,
    b: 1.,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = color![1., 2., 3.];
        let v2 = color![4., 5., 6.];
        assert_eq!(v1 + v2, color![5., 7., 9.])
    }

    #[test]
    fn sub() {
        let v1 = color![1., 2., 3.];
        let v2 = color![4., 5., 6.];
        assert_eq!(v2 - v1, color![3., 3., 3.])
    }

    #[test]
    fn mul() {
        let v = color![1., 2., 3.];
        assert_eq!(3. * v, v * 3.);
        assert_eq!(3. * v, color![3., 6., 9.]);
    }

    #[test]
    fn div() {
        let v = color![1., 2., 3.];
        assert_eq!(v / 2., color![0.5, 1., 1.5]);
    }

    #[test]
    #[should_panic]
    #[allow(clippy::no_effect)]
    fn index() {
        let v = color![1., 2., 3.];
        v[3];
    }
}
