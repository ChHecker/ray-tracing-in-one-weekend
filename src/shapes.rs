use std::sync::Arc;

use crate::{materials::Material, *};

pub trait Position {}

pub struct Stationary {
    pub position: Point,
}
impl Position for Stationary {}

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

type MaterialArc = Arc<dyn Material>;

pub struct Sphere<P: Position> {
    center: P,
    radius: f32,
    material: MaterialArc,
}

impl Sphere<Stationary> {
    pub fn new(center: Point, radius: f32, material: MaterialArc) -> Self {
        Self {
            center: Stationary { position: center },
            radius,
            material,
        }
    }

    pub fn with_time(self, position_end: Point, time_start: f32, time_end: f32) -> Sphere<Moving> {
        Sphere::<Moving> {
            center: Moving {
                position: (self.center.position, position_end),
                time: (time_start, time_end),
            },
            radius: self.radius,
            material: self.material,
        }
    }
}

impl Hittable for Sphere<Stationary> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center.position;
        let a = ray.direction().norm_sq();
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_sq() - self.radius.powi(2);
        let discriminant = b_halves.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (-b_halves - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-b_halves + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord::from_ray(
            point,
            (point - self.center.position) / self.radius,
            root,
            self.material.clone(),
            ray,
        ))
    }
}

impl Hittable for Sphere<Moving> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center.position(ray.time());
        let a = ray.direction().norm_sq();
        let b_halves = oc.dot(&ray.direction());
        let c = oc.norm_sq() - self.radius.powi(2);
        let discriminant = b_halves.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (-b_halves - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-b_halves + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord::from_ray(
            point,
            (point - self.center.position(ray.time())) / self.radius,
            root,
            self.material.clone(),
            ray,
        ))
    }
}

pub struct Cylinder {
    center: Point,
    radius: f32,
    height: f32,
    material: MaterialArc,
}

impl Cylinder {
    pub fn new(center: Point, radius: f32, height: f32, material: MaterialArc) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = point!(
            ray.origin().x() - self.center.x(),
            0.,
            ray.origin().z() - self.center.z(),
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
        if (root1 < t_min || t_max < root1) && (root2 < t_min || t_max < root2) {
            return None;
        }

        let point1 = ray.at(root1);
        let point2 = ray.at(root2);

        let upper_bound = self.center.y() + self.height / 2.;
        let lower_bound = self.center.y() - self.height / 2.;

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

            if root < t_min || t_max < root {
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

            if root < t_min || t_max < root {
                return None;
            }
        } else {
            root = root1;
            point = point1;
            if root < t_min || t_max < root {
                root = root2;
                point = point2;
                if root < t_min || t_max < root {
                    return None;
                }
            }
        }

        let mut normal = (point - self.center) / self.radius;
        normal = point!(normal.x(), 0., normal.z());

        Some(HitRecord::from_ray(
            point,
            normal,
            root,
            self.material.clone(),
            ray,
        ))
    }
}
