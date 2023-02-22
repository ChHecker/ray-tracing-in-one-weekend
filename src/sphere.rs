use crate::{Point3, Ray, Vec3};

pub fn hits_sphere(ray: &Ray, center: &Point3, radius: f64) -> bool {
    let oc = ray.origin() - *center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2. * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.
}
