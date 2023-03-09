//! Collection of [hittable](`Hittable`) shapes .

use crate::{hittable::Aabb, materials::Material, ray::Ray, *};
use std::{fmt::Debug, sync::Arc};

/// Zero-size trait to mark shapes as [stationary](Stationary) or [moving](Moving).
pub trait Position: Debug {}

#[derive(Debug)]
pub struct Stationary {
    pub position: Point,
}
impl Stationary {
    pub fn position(&self) -> Point {
        self.position
    }
}
impl Position for Stationary {}

#[derive(Debug)]
pub struct Moving {
    pub position: (Point, Point),
    pub time: (f32, f32),
}
impl Moving {
    pub fn position(&self, time: f32) -> Point {
        self.position.0
            + ((time - self.time.0) / (self.time.1 - self.time.0))
                * (self.position.1 - self.position.0)
    }
}
impl Position for Moving {}

/// A sphere, either [stationary](Stationary) or [moving](Moving).
///
/// # Fields
/// - `center`: Center of the sphere.
/// - `radius`: Radius of the sphere.
/// - `material`: Material of the sphere.
#[derive(Debug)]
pub struct Sphere<M: Material + 'static, P: Position> {
    center: P,
    radius: f32,
    material: Arc<M>,
}

impl<M: Material, P: Position> Sphere<M, P> {
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn material(&self) -> Arc<M> {
        self.material.clone()
    }
}

impl<M: Material> Sphere<M, Stationary> {
    /// Create a new [stationary](Stationary) [`Sphere`].
    pub fn new(center: Point, radius: f32, material: Arc<M>) -> Self {
        Self {
            center: Stationary { position: center },
            radius,
            material,
        }
    }

    /// Consume `self` and create a [moving](Moving) [`Camera`].
    pub fn with_time(
        self,
        position_end: Point,
        time_start: f32,
        time_end: f32,
    ) -> Sphere<M, Moving> {
        Sphere {
            center: Moving {
                position: (self.center.position, position_end),
                time: (time_start, time_end),
            },
            radius: self.radius,
            material: self.material,
        }
    }

    pub fn position(&self) -> Point {
        self.center.position()
    }
}

impl<M: Material> Sphere<M, Moving> {
    pub fn position(&self, time: f32) -> Point {
        self.center.position(time)
    }
}

impl<M: Material> Hittable for Sphere<M, Stationary> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.position();
        let a = ray.direction().norm_sq();
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_sq() - self.radius.powi(2);
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
        Some(HitRecord::from_ray(
            point,
            (point - self.position()) / self.radius,
            root,
            self.material.clone(),
            ray,
        ))
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.position() - point![self.radius, self.radius, self.radius],
            self.position() + point![self.radius, self.radius, self.radius],
        ))
    }
}

impl<M: Material> Hittable for Sphere<M, Moving> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.position(ray.time());
        let a = ray.direction().norm_sq();
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_sq() - self.radius.powi(2);
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
        Some(HitRecord::from_ray(
            point,
            (point - self.position(ray.time())) / self.radius,
            root,
            self.material.clone(),
            ray,
        ))
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        let aabb1 = Aabb::new(
            self.position(time0) - point![self.radius, self.radius, self.radius],
            self.position(time0) + point![self.radius, self.radius, self.radius],
        );
        let aabb2 = Aabb::new(
            self.position(time1) - point![self.radius, self.radius, self.radius],
            self.position(time1) + point![self.radius, self.radius, self.radius],
        );
        Some(Aabb::surrounding(&aabb1, &aabb2))
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
#[derive(Debug)]
pub struct Cylinder<M: Material + 'static, P: Position> {
    center: P,
    radius: f32,
    height: f32,
    material: Arc<M>,
}

impl<M: Material, P: Position> Cylinder<M, P> {
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn material(&self) -> Arc<M> {
        self.material.clone()
    }
}

impl<M: Material> Cylinder<M, Stationary> {
    /// Create a new [stationary](Stationary) [`Cylinder`].
    pub fn new(center: Point, radius: f32, height: f32, material: Arc<M>) -> Self {
        Self {
            center: Stationary { position: center },
            radius,
            height,
            material,
        }
    }

    pub fn position(&self) -> Point {
        self.center.position()
    }
}

impl<M: Material> Hittable for Cylinder<M, Stationary> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = point!(
            ray.origin().x() - self.position().x(),
            0.,
            ray.origin().z() - self.position().z(),
        );
        let a = ray.direction().x().powi(2) + ray.direction().z().powi(2);
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_sq() - self.radius.powi(2);
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

        let upper_bound = self.position().y() + self.height / 2.;
        let lower_bound = self.position().y() - self.height / 2.;

        let mut point: Point;
        let mut root: f32;

        if point1.y() > upper_bound {
            if point2.y() > upper_bound {
                return None;
            }
            if ray.direction().y() == 0. {
                return None;
            }

            root = (upper_bound - ray.origin().y()) / ray.direction().y();
            point = ray.at(root);

            if root < t_min || root > t_max {
                return None;
            }
        } else if point1.y() < lower_bound {
            if point2.y() < lower_bound {
                return None;
            }
            if ray.direction().y() == 0. {
                return None;
            }

            root = (lower_bound - ray.origin().y()) / ray.direction().y();
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

        let mut normal = (point - self.position()) / self.radius;
        normal = point!(normal.x(), 0., normal.z());

        Some(HitRecord::from_ray(
            point,
            normal,
            root,
            self.material.clone(),
            ray,
        ))
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.position() - point![self.radius, self.height / 2., self.radius],
            self.position() + point![self.radius, self.height / 2., self.radius],
        ))
    }
}
