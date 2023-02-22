use crate::{
    hittable::{HitRecord, Hittable},
    Point3, Ray, Vec3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        Some(HitRecord::new(
            point,
            (point - self.center) / self.radius,
            root,
        ))
    }
}
