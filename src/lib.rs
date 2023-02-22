pub mod camera;
pub mod hittable;
pub mod ppm;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use ppm::write_ppm;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color3, Point3, Vec3};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
