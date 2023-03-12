//! Collection of [hittable](`Hittable`) shapes .

use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::Debug;

use nalgebra::Rotation3;
use rand::Rng;

use crate::hitrecord::HitRecord;
use crate::hittable::Aabb;
use crate::materials::Material;
use crate::ray::Ray;
use crate::*;

/// Marks an object to support movement and rotation via [`Offset`].
pub trait Movable: Clone + Debug + Hittable {
    /// Consumes `self` and returns a rotated version.
    fn with_rotation(self, rotation: Rotation3<f32>) -> Self;

    /// Consumes `self` and returns a moving version (translatory).
    fn moving(self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self;
}

/// Marks an object as moving (translatory).
#[derive(Clone, Default, Debug)]
struct Moving {
    pub offset_end: Vector3<f32>,
    pub time_start: f32,
    pub time_end: f32,
}

#[derive(Clone, Default, Debug)]
pub struct Offset {
    offset_start: Vector3<f32>,
    rotation: Option<Rotation3<f32>>,
    moving: Option<Moving>,
}
impl Offset {
    pub fn new(offset: Vector3<f32>) -> Self {
        Self {
            offset_start: offset,
            rotation: None,
            moving: None,
        }
    }

    pub fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.moving = Some(Moving {
            offset_end,
            time_start,
            time_end,
        });
        self
    }

    fn offset(&self, time: f32) -> Vector3<f32> {
        match &self.moving {
            Some(moving) => {
                self.offset_start
                    + ((time - moving.time_start) / (moving.time_end - moving.time_start))
                        * (moving.offset_end - self.offset_start)
            }
            None => self.offset_start,
        }
    }

    pub(crate) fn hit<'a, H: Hittable + ?Sized>(
        &'a self,
        hittable: &'a H,
        ray: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        // Rotation
        let rotated_ray = match self.rotation {
            Some(rotation) => {
                Ray::new(rotation * ray.origin(), rotation * ray.direction()).with_time(ray.time())
            }
            None => ray,
        };

        // Translation
        let offset_ray = Ray::new(
            rotated_ray.origin() - self.offset(rotated_ray.time()),
            rotated_ray.direction(),
        )
        .with_time(rotated_ray.time());

        let mut hit_record_option = hittable.hit_origin(offset_ray, t_min, t_max);

        if let Some(hit_record) = &mut hit_record_option {
            hit_record.point += self.offset(ray.time());
            if let Some(rotation) = self.rotation {
                hit_record.point = rotation.inverse() * hit_record.point;
                hit_record.normal = rotation.inverse() * hit_record.normal;
            }
        }

        hit_record_option
    }

    pub(crate) fn bounding_box<'a, H: Hittable + ?Sized>(
        &'a self,
        hittable: &'a H,
        time0: f32,
        time1: f32,
    ) -> Option<Aabb> {
        let mut aabb_option = hittable.bounding_box_origin(time0, time1);
        if let Some(aabb) = &mut aabb_option {
            if let Some(rotation) = self.rotation {
                aabb.minimum = rotation * aabb.minimum;
                aabb.maximum = rotation * aabb.maximum;
            }
            aabb.minimum += self.offset(time0);
            aabb.maximum += self.offset(time1);
        }

        aabb_option
    }
}

/// A sphere, either [stationary](Stationary) or [moving](Moving).
///
/// # Fields
/// - `center`: Center of the sphere.
/// - `radius`: Radius of the sphere.
/// - `material`: Material of the sphere.
#[derive(Clone, Debug)]
pub struct Sphere<M: Material> {
    center: Offset,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    /// Create a new [stationary](Stationary) [`Sphere`].
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self {
        Self {
            center: Offset::new(center),
            radius,
            material,
        }
    }

    pub fn position(&self, time: f32) -> Vector3<f32> {
        self.center.offset(time)
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn material(&self) -> &M {
        &self.material
    }

    /// Get the surface coordinates (u, v) on the sphere from a [`Vector3<f32>`].
    ///
    /// The pair (u, v) is defined by the angles in spherical coordinates via u = phi/(2pi), v = theta/pi.
    fn get_surface_coordinates(&self, point: Vector3<f32>) -> (f32, f32) {
        let phi = point.z.atan2(point.x);
        let theta = point.y.asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + FRAC_PI_2) / PI;
        (u, v)
    }
}

impl<M: Material + Clone + 'static> Hittable for Sphere<M> {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin();
        let a = ray.direction().norm_squared();
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius.powi(2);
        let discriminant = b_halves.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (-b_halves - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-b_halves + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = point / self.radius;
        let (u, v) = self.get_surface_coordinates(normal);

        Some(HitRecord::from_ray(
            point,
            u,
            v,
            normal,
            root,
            self.material(),
            ray,
        ))
    }

    fn bounding_box_origin(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            -vector![self.radius.abs(), self.radius.abs(), self.radius.abs()],
            vector![self.radius.abs(), self.radius.abs(), self.radius.abs()],
        ))
    }

    fn center(&self) -> &Offset {
        &self.center
    }
}

impl<M: Material + Clone + 'static> Movable for Sphere<M> {
    fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.center = self.center.with_rotation(rotation);
        self
    }

    fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.center = self.center.moving(offset_end, time_start, time_end);
        self
    }
}

/// A cylinder along the y axis.
///
/// Right now, the cylinder cannot move
///
/// # Fields
/// - `center`: Center of the cylinder.
/// - `radius`: Radius of the cylinder.
/// - `height`: Height of the cylinder (from top to bottom, not from center to bottom).
/// - `material`: Material of the cylinder.
#[derive(Clone, Debug)]
pub struct Cylinder<M: Material> {
    center: Offset,
    radius: f32,
    height: f32,
    material: M,
}

impl<M: Material> Cylinder<M> {
    /// Create a new [stationary](Stationary) [`Cylinder`].
    pub fn new(center: Vector3<f32>, radius: f32, height: f32, material: M) -> Self {
        Self {
            center: Offset::new(center),
            radius,
            height,
            material,
        }
    }

    pub fn moving(self, position_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        let center = self.center.moving(position_end, time_start, time_end);
        Self {
            center,
            radius: self.radius,
            height: self.height,
            material: self.material,
        }
    }

    pub fn position(&self, time: f32) -> Vector3<f32> {
        self.center.offset(time)
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn material(&self) -> &M {
        &self.material
    }
}

impl<M: Material + Clone + 'static> Hittable for Cylinder<M> {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = vector![ray.origin().x, 0., ray.origin().z];
        let a = ray.direction().x.powi(2) + ray.direction().z.powi(2);
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius.powi(2);
        let discriminant = b_halves.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let root1 = (-b_halves - discriminant_sqrt) / a;
        let root2 = (-b_halves + discriminant_sqrt) / a;
        if (root1 < t_min || root1 > t_max) && (root2 < t_min || root2 > t_max) {
            return None;
        }

        let point1 = ray.at(root1);
        let point2 = ray.at(root2);

        let upper_bound = self.height / 2.;
        let lower_bound = -self.height / 2.;

        let mut point: Vector3<f32>;
        let mut root: f32;

        if point1.y > upper_bound {
            if point2.y > upper_bound {
                return None;
            }
            if ray.direction().y == 0. {
                return None;
            }

            root = (upper_bound - ray.origin().y) / ray.direction().y;
            point = ray.at(root);

            if root < t_min || root > t_max {
                return None;
            }
        } else if point1.y < lower_bound {
            if point2.y < lower_bound {
                return None;
            }
            if ray.direction().y == 0. {
                return None;
            }

            root = (lower_bound - ray.origin().y) / ray.direction().y;
            point = ray.at(root);

            if root < t_min || root > t_max {
                return None;
            }
        } else {
            root = root1;
            point = point1;
            if root < t_min || root > t_max {
                root = root2;
                point = point2;
                if root < t_min || root > t_max {
                    return None;
                }
            }
        }

        let mut normal = point / self.radius;
        normal = vector!(normal.x, 0., normal.z);

        Some(HitRecord::from_ray(
            point,
            0., // TODO: Parametrization of Cylinder
            0.,
            normal,
            root,
            &self.material,
            ray,
        ))
    }

    fn bounding_box_origin(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            -vector![self.radius.abs(), self.height.abs() / 2., self.radius.abs()],
            vector![self.radius.abs(), self.height.abs() / 2., self.radius.abs()],
        ))
    }

    fn center(&self) -> &Offset {
        &self.center
    }
}

impl<M: Material + Clone + 'static> Movable for Cylinder<M> {
    fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.center = self.center.with_rotation(rotation);
        self
    }

    fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.center = self.center.moving(offset_end, time_start, time_end);
        self
    }
}

#[derive(Clone, Debug)]
pub enum Plane {
    XY,
    YZ,
    XZ,
}

/// Axis-aligned planes.
impl Plane {
    /// Return the indices of the axes along the [`Plane`].
    pub fn axes(&self) -> (usize, usize, usize) {
        match self {
            Plane::XY => (0, 1, 2),
            Plane::YZ => (1, 2, 0),
            Plane::XZ => (0, 2, 1),
        }
    }
}

/// A flat rectangle along the x y plane.
///
/// # Fields
/// - `orientation`: Along which [`Plane`] the [`Rectangle`] should be oriented.
/// - `center`: Its center.
/// - `width`: Its width, defined along the first of the two axes of the [`Plane`].
/// - `height`: Its height, defined along the second of the two axes of the [`Plane`].
/// - `material`: Its material.
#[derive(Clone, Debug)]
pub struct Rectangle<M: Material> {
    orientation: Plane,
    center: Offset,
    width: f32,
    height: f32,
    material: M,
}

impl<M: Material> Rectangle<M> {
    pub fn new(
        orientation: Plane,
        center: Vector3<f32>,
        width: f32,
        height: f32,
        material: M,
    ) -> Self {
        let center = Offset::new(center);
        Self {
            orientation,
            center,
            width,
            height,
            material,
        }
    }

    pub fn xy(center: Vector3<f32>, width: f32, height: f32, material: M) -> Self {
        let orientation = Plane::XY;
        let center = Offset::new(center);
        Self {
            orientation,
            center,
            width,
            height,
            material,
        }
    }

    pub fn yz(center: Vector3<f32>, width: f32, height: f32, material: M) -> Self {
        let orientation = Plane::YZ;
        let center = Offset::new(center);
        Self {
            orientation,
            center,
            width,
            height,
            material,
        }
    }

    pub fn xz(center: Vector3<f32>, width: f32, height: f32, material: M) -> Self {
        let orientation = Plane::XZ;
        let center = Offset::new(center);
        Self {
            orientation,
            center,
            width,
            height,
            material,
        }
    }

    pub fn moving(self, position_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        let center = self.center.moving(position_end, time_start, time_end);
        Self {
            orientation: self.orientation,
            center,
            width: self.width,
            height: self.height,
            material: self.material,
        }
    }

    pub fn position(&self, time: f32) -> Vector3<f32> {
        self.center.offset(time)
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn material(&self) -> &M {
        &self.material
    }
}

impl<M: Material + Clone + 'static> Hittable for Rectangle<M> {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (a_index, b_index, c_index) = self.orientation.axes();
        let a_min = -self.width / 2.;
        let a_max = self.width / 2.;
        let b_min = -self.height / 2.;
        let b_max = self.height / 2.;

        let t = -ray.origin()[c_index] / ray.direction()[c_index];
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let a = point[a_index];
        let b = point[b_index];
        if a < a_min || a > a_max || b < b_min || b > b_max {
            return None;
        }

        let u = (a - a_min) / (a_max - a_min);
        let v = (b - b_min) / (b_max - b_min);
        let mut normal = vector![0., 0., 0.];
        normal[c_index] = 1.;

        Some(HitRecord::from_ray(
            point,
            u,
            v,
            normal,
            t,
            &self.material,
            ray,
        ))
    }

    fn bounding_box_origin(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        let minimum = -vector![self.width / 2., self.height / 2., 0.0001];
        let maximum = vector![self.width / 2., self.height / 2., 0.0001];
        Some(Aabb::new(minimum, maximum))
    }

    fn center(&self) -> &Offset {
        &self.center
    }
}

impl<M: Material + Clone + 'static> Movable for Rectangle<M> {
    fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.center = self.center.with_rotation(rotation);
        self
    }

    fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.center = self.center.moving(offset_end, time_start, time_end);
        self
    }
}

/// A axis-aligned parallelepipied (3D rectangle).
///
/// # Fields:
/// - `center`: Its [`Offset`].
/// - `width`: Its width (in x direction).
/// - `height`: Its height (in y direction).
/// - `depth`: Its depth (in z direction).
/// - `material`: Its material.
#[derive(Clone, Debug)]
pub struct Parallelepiped<M: Material> {
    center: Offset,
    width: f32,
    height: f32,
    depth: f32,
    rectangles: HittableList,
    material: M,
}

impl<M: Material + Clone + 'static> Parallelepiped<M> {
    pub fn new(center: Vector3<f32>, width: f32, height: f32, depth: f32, material: M) -> Self {
        let mut rectangles = HittableList::new();

        let bottom = Rectangle::xz(
            -vector![0., height / 2., 0.],
            width,
            depth,
            material.clone(),
        );
        let top = Rectangle::xz(vector![0., height / 2., 0.], width, depth, material.clone());
        let left = Rectangle::yz(
            -vector![width / 2., 0., 0.],
            height,
            depth,
            material.clone(),
        );
        let right = Rectangle::yz(vector![width / 2., 0., 0.], height, depth, material.clone());
        let back = Rectangle::xy(
            -vector![0., 0., depth / 2.],
            width,
            height,
            material.clone(),
        );
        let front = Rectangle::xy(vector![0., 0., depth / 2.], width, height, material.clone());
        rectangles.push(top);
        rectangles.push(bottom);
        rectangles.push(left);
        rectangles.push(right);
        rectangles.push(back);
        rectangles.push(front);

        let center = Offset::new(center);

        Self {
            center,
            width,
            height,
            depth,
            rectangles,
            material,
        }
    }

    pub fn position(&self, time: f32) -> Vector3<f32> {
        self.center.offset(time)
    }

    pub fn material(&self) -> &M {
        &self.material
    }
}

impl<M: Material + Clone + 'static> Hittable for Parallelepiped<M> {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.rectangles.hit(ray, t_min, t_max)
    }

    fn bounding_box_origin(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            -vector![self.width / 2., self.height / 2., self.depth / 2.],
            vector![self.width / 2., self.height / 2., self.depth / 2.],
        ))
    }

    fn center(&self) -> &Offset {
        &self.center
    }
}

impl<M: Material + Clone + 'static> Movable for Parallelepiped<M> {
    fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.center = self.center.with_rotation(rotation);
        self
    }

    fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.center = self.center.moving(offset_end, time_start, time_end);
        self
    }
}

/// A medium of constant optical density.
#[derive(Clone, Debug)]
pub struct ConstantMedium<H: Hittable, M: Material> {
    boundary: H,
    material: M,
    negative_inverse_density: f32,
}

impl<H: Hittable, M: Material> ConstantMedium<H, M> {
    pub fn new(boundary: H, material: M, density: f32) -> Self {
        Self {
            boundary,
            material,
            negative_inverse_density: -1. / density,
        }
    }

    pub fn material(&self) -> &M {
        &self.material
    }
}

impl<H, M> Hittable for ConstantMedium<H, M>
where
    H: Hittable + Clone + 'static,
    M: Material + Clone + 'static,
{
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        let mut hit1 = match self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY) {
            Some(hit) => hit,
            None => return None,
        };
        let mut hit2 = match self.boundary.hit(ray, hit1.t + 0.0001, f32::INFINITY) {
            Some(hit) => hit,
            None => return None,
        };

        if hit1.t < t_min {
            hit1.t = t_min
        };
        if hit2.t > t_max {
            hit2.t = t_max
        };

        if hit1.t > hit2.t {
            return None;
        }

        if hit1.t < 0. {
            hit1.t = 0.
        }

        let ray_length = ray.direction().norm();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rng.gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        let point = ray.at(t);
        Some(HitRecord::new(
            point,
            0.,
            0.,
            Vector3::zeros(),
            t,
            true,
            &self.material,
        ))
    }

    fn bounding_box_origin(&self, time0: f32, time1: f32) -> Option<Aabb> {
        self.boundary.bounding_box_origin(time0, time1)
    }

    fn center(&self) -> &Offset {
        self.boundary.center()
    }
}
