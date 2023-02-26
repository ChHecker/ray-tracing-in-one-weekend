use crate::clamp;
use image::Rgb;
use rand::Rng;
use std::{fmt, ops};

/// Three-dimensional Cartesian vector
pub trait Vec3: Copy + Clone + fmt::Debug + PartialEq {
    fn new(x: f64, y: f64, z: f64) -> Self;

    fn random() -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    fn random_in_range(min: f64, max: f64) -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        Self::new(
            min + rng.gen::<f64>() * (max - min),
            min + rng.gen::<f64>() * (max - min),
            min + rng.gen::<f64>() * (max - min),
        )
    }

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross(&self, rhs: &Self) -> Self
    where
        Self: Sized,
    {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    fn norm_sq(&self) -> f64 {
        self.dot(self)
    }

    fn unit_vector(&self) -> Self
    where
        Self: Sized,
    {
        let abs = self.norm();
        Self::new(self.x() / abs, self.y() / abs, self.z() / abs)
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
}

/// Color in RGB between 0 and 1
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn to_color_str(&self) -> String {
        format!(
            "{} {} {}",
            (256. * clamp(self.x(), 0., 0.999)) as u8,
            (256. * clamp(self.y(), 0., 0.999)) as u8,
            (256. * clamp(self.z(), 0., 0.999)) as u8
        )
    }
}

impl Vec3 for Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Color(r, g, b)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (256. * clamp(self.x(), 0., 0.999)) as u8,
            (256. * clamp(self.y(), 0., 0.999)) as u8,
            (256. * clamp(self.z(), 0., 0.999)) as u8,
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

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
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

impl ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Color(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
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
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bound"),
        }
    }
}

pub struct Color3Iter {
    index: usize,
    col3: Color,
}

impl IntoIterator for Color {
    type Item = f64;
    type IntoIter = Color3Iter;

    fn into_iter(self) -> Self::IntoIter {
        Color3Iter {
            index: 0,
            col3: self,
        }
    }
}

impl Iterator for Color3Iter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.col3.x(),
            1 => self.col3.y(),
            2 => self.col3.z(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl FromIterator<f64> for Color {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }
}

/// Vector in 3D space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3(f64, f64, f64);

impl Point3 {
    pub fn to_color_str(&self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let rand = Self::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
            if rand.norm_sq() < 1. {
                return rand;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let rand = Self::random_in_unit_sphere();
        if rand.dot(&normal) > 0. {
            return rand;
        }
        -rand
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let rand = Point3::new(-1. + rng.gen::<f64>() * 2., -1. + rng.gen::<f64>() * 2., 0.);
            if rand.norm_sq() < 1. {
                return rand;
            }
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2. * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(-self.dot(normal), 1.);
        let refracted_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let refracted_out_parallel = -(1. - refracted_out_perp.norm_sq()).abs().sqrt() * *normal;
        refracted_out_perp + refracted_out_parallel
    }
}

impl Vec3 for Point3 {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Point3(r, g, b)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }
}

impl ops::Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point3 {
    type Output = Self;

    fn sub(self, rhs: Point3) -> Self::Output {
        Point3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Point3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point3(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl ops::Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = rhs * *self;
    }
}

impl ops::Div<f64> for Point3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Point3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Point3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl ops::Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point3(-self.0, -self.1, -self.2)
    }
}

impl ops::Index<u8> for Point3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bound"),
        }
    }
}

pub struct Point3Iter {
    index: usize,
    point3: Point3,
}

impl IntoIterator for Point3 {
    type Item = f64;
    type IntoIter = Point3Iter;

    fn into_iter(self) -> Self::IntoIter {
        Point3Iter {
            index: 0,
            point3: self,
        }
    }
}

impl Iterator for Point3Iter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.point3.x(),
            1 => self.point3.y(),
            2 => self.point3.z(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Color(1., 2., 3.);
        let v2 = Color(4., 5., 6.);
        assert_eq!(v1 + v2, Color(5., 7., 9.))
    }

    #[test]
    fn sub() {
        let v1 = Color(1., 2., 3.);
        let v2 = Color(4., 5., 6.);
        assert_eq!(v2 - v1, Color(3., 3., 3.))
    }

    #[test]
    fn mul() {
        let v = Color(1., 2., 3.);
        assert_eq!(3. * v, v * 3.);
        assert_eq!(3. * v, Color(3., 6., 9.));
    }

    #[test]
    fn div() {
        let v = Color(1., 2., 3.);
        assert_eq!(v / 2., Color(0.5, 1., 1.5));
    }

    #[test]
    fn dot() {
        let v1 = Color(1., 2., 3.);
        let v2 = Color(4., 5., 6.);
        assert_eq!(v1.dot(&v2), 32.)
    }

    #[test]
    fn cross() {
        let v1 = Color(1., 2., 3.);
        let v2 = Color(4., 5., 6.);
        assert_eq!(v1.cross(&v2), Color(-3., 6., -3.))
    }

    #[test]
    fn abs() {
        let v = Color(1., 2., 3.);
        assert_eq!(v.norm(), f64::sqrt(14.))
    }

    #[test]
    fn unit_vector() {
        let v = Color(1., 2., 3.);
        assert_eq!(v.unit_vector(), v / f64::sqrt(14.))
    }

    #[test]
    #[should_panic]
    fn index() {
        let v = Color(1., 2., 3.);
        v[3];
    }
}
