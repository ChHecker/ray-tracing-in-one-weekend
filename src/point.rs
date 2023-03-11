use std::ops;

use rand::Rng;

use crate::*;

/// Macro for [`Point::new`]
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr $(,)*) => {
        Point::new($x, $y, $z)
    };
}

/// Vector in 3D Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    /// Dot product.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v1 = point![1., 2., 3.];
    /// let v2 = point![4., 5., 6.];
    /// assert_eq!(v1.dot(&v2), 32.)
    /// ```
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    /// Cross product.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v1 = point![1., 2., 3.];
    /// let v2 = point![4., 5., 6.];
    /// assert_eq!(v1.cross(&v2), point![-3., 6., -3.])
    /// ```
    pub fn cross(&self, rhs: &Self) -> Self
    where
        Self: Sized,
    {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    /// Cartesian norm of the vector.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1., 2., 3.];
    /// assert_eq!(v.norm(), (14f32).sqrt())
    /// ```
    pub fn norm(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    /// Square of the Cartesian norm of the vector.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1., 2., 3.];
    /// assert_eq!(v.norm_sq(), 14.)
    /// ```
    pub fn norm_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Unit vector.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1., 2., 3.];
    /// assert_eq!(v.unit_vector(), v / f32::sqrt(14.))
    /// ```
    pub fn unit_vector(&self) -> Self
    where
        Self: Sized,
    {
        let abs = self.norm();
        Self::new(self.x() / abs, self.y() / abs, self.z() / abs)
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let rand = Self::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
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
        if rand.dot(normal) > 0. {
            return rand;
        }
        -rand
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let rand = Point::new(-1. + rng.gen::<f32>() * 2., -1. + rng.gen::<f32>() * 2., 0.);
            if rand.norm_sq() < 1. {
                return rand;
            }
        }
    }

    /// Reflects the vector.
    ///
    /// The reflection follows the rule of equal angles with respect to `normal`.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1., 2., 3.].unit_vector();
    /// let normal = v;
    /// assert!((v.reflect(&normal) + v).norm() < 0.01);
    /// ```
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2. * self.dot(normal) * *normal
    }

    /// Refracts the vector.
    ///
    /// The refraction follows Snell's law with the fraction of refraction indices `etai_over_etat`.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1., 2., 3.].unit_vector();
    /// let normal = v;
    /// eprintln!("{:?}", v.refract(&normal, 1.));
    /// assert!((v.refract(&normal, 1.) + normal).norm() < 0.01);
    /// ```
    pub fn refract(&self, normal: &Self, etai_over_etat: f32) -> Self {
        let cos_theta = f32::min(-self.dot(normal), 1.);
        let refracted_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let refracted_out_parallel = -(1. - refracted_out_perp.norm_sq()).abs().sqrt() * *normal;
        refracted_out_perp + refracted_out_parallel
    }

    /// Checks whether a vector is near 0 (up to 1e-8) in every component.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, vec3::*};
    /// let v = point![1e-9, 1e-9, 1e-9];
    /// assert_eq!(v.near_zero(), true);
    /// ```
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
}

impl Vec3 for Point {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Point::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = rhs * *self;
    }
}

impl ops::Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Index<u8> for Point {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bound"),
        }
    }
}

impl ops::IndexMut<u8> for Point {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bound"),
        }
    }
}

/// Iterator over the coordinates of [`Point`].
pub struct Point3Iter {
    index: usize,
    point3: Point,
}

impl IntoIterator for Point {
    type Item = f32;
    type IntoIter = Point3Iter;

    fn into_iter(self) -> Self::IntoIter {
        Point3Iter {
            index: 0,
            point3: self,
        }
    }
}

impl Iterator for Point3Iter {
    type Item = f32;

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
        let v1 = point![1., 2., 3.];
        let v2 = point![4., 5., 6.];
        assert_eq!(v1 + v2, point![5., 7., 9.])
    }

    #[test]
    fn sub() {
        let v1 = point![1., 2., 3.];
        let v2 = point![4., 5., 6.];
        assert_eq!(v2 - v1, point![3., 3., 3.])
    }

    #[test]
    fn mul() {
        let v = point![1., 2., 3.];
        assert_eq!(3. * v, v * 3.);
        assert_eq!(3. * v, point![3., 6., 9.]);
    }

    #[test]
    fn div() {
        let v = point![1., 2., 3.];
        assert_eq!(v / 2., point![0.5, 1., 1.5]);
    }

    #[test]
    #[should_panic]
    fn index() {
        let v = point![1., 2., 3.];
        v[3];
    }
}
