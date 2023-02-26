use std::sync::Arc;

use crate::*;

type MaterialArc = Arc<dyn Material>;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: MaterialArc,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialArc) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
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
            (point - self.center) / self.radius,
            root,
            self.material.clone(),
            ray,
        ))
    }
}

pub struct ZCylinder {
    center: Point3,
    radius: f64,
    height: f64,
    material: MaterialArc,
}

impl ZCylinder {
    pub fn new(center: Point3, radius: f64, height: f64, material: MaterialArc) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }
}

impl Hittable for ZCylinder {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = Point3::new(
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

        let mut point: Point3;
        let mut root: f64;

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
        normal = Point3::new(normal.x(), 0., normal.z());

        Some(HitRecord::from_ray(
            point,
            normal,
            root,
            self.material.clone(),
            ray,
        ))
    }
}
