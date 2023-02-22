use crate::{Point3, Ray, Vec3};

pub fn hits_sphere(ray: &Ray, center: &Point3, radius: f64) -> Option<f64> {
    let oc = ray.origin() - *center;
    let a = ray.direction().norm_sq();
    let b_halves = oc.dot(&ray.direction());
    let c = oc.norm_sq() - radius.powi(2);
    let discriminant = b_halves.powi(2) - a * c;
    if discriminant < 0. {
        return None;
    }
    Some((-b_halves - discriminant.sqrt()) / a)
}
