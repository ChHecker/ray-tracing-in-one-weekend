pub mod hittable;
pub mod ppm;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use hittable::{HitRecord, Hittable, HittableList};
pub use ppm::write_ppm;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color3, Point3, Vec3};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
