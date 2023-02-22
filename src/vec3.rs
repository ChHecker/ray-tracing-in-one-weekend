use std::ops;

/// Three-dimensional Cartesian vector
pub trait Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self;

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
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color3(f64, f64, f64);

impl Color3 {
    pub fn to_color_str(&self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8
        )
    }
}

impl Vec3 for Color3 {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Color3(r, g, b)
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

impl ops::Add<Color3> for Color3 {
    type Output = Self;

    fn add(self, rhs: Color3) -> Self::Output {
        Color3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Color3> for Color3 {
    fn add_assign(&mut self, rhs: Color3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Color3> for Color3 {
    type Output = Self;

    fn sub(self, rhs: Color3) -> Self::Output {
        Color3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign<Color3> for Color3 {
    fn sub_assign(&mut self, rhs: Color3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Color3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color3(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl ops::Mul<Color3> for f64 {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Self::Output {
        Color3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::MulAssign<f64> for Color3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = rhs * *self;
    }
}

impl ops::Div<f64> for Color3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Color3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Color3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl ops::Neg for Color3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color3(-self.0, -self.1, -self.2)
    }
}

impl ops::Index<u8> for Color3 {
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
    col3: Color3,
}

impl IntoIterator for Color3 {
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

impl ops::Add<Point3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Point3> for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Point3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Point3) -> Self::Output {
        Point3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign<Point3> for Point3 {
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
        let v1 = Color3(1., 2., 3.);
        let v2 = Color3(4., 5., 6.);
        assert_eq!(v1 + v2, Color3(5., 7., 9.))
    }

    #[test]
    fn sub() {
        let v1 = Color3(1., 2., 3.);
        let v2 = Color3(4., 5., 6.);
        assert_eq!(v2 - v1, Color3(3., 3., 3.))
    }

    #[test]
    fn mul() {
        let v = Color3(1., 2., 3.);
        assert_eq!(3. * v, v * 3.);
        assert_eq!(3. * v, Color3(3., 6., 9.));
    }

    #[test]
    fn div() {
        let v = Color3(1., 2., 3.);
        assert_eq!(v / 2., Color3(0.5, 1., 1.5));
    }

    #[test]
    fn dot() {
        let v1 = Color3(1., 2., 3.);
        let v2 = Color3(4., 5., 6.);
        assert_eq!(v1.dot(&v2), 32.)
    }

    #[test]
    fn cross() {
        let v1 = Color3(1., 2., 3.);
        let v2 = Color3(4., 5., 6.);
        assert_eq!(v1.cross(&v2), Color3(-3., 6., -3.))
    }

    #[test]
    fn abs() {
        let v = Color3(1., 2., 3.);
        assert_eq!(v.norm(), f64::sqrt(14.))
    }

    #[test]
    fn unit_vector() {
        let v = Color3(1., 2., 3.);
        assert_eq!(v.unit_vector(), v / f64::sqrt(14.))
    }

    #[test]
    #[should_panic]
    fn index() {
        let v = Color3(1., 2., 3.);
        v[3];
    }
}
