pub mod camera;
pub mod hittable;
pub mod materials;
pub mod ppm;
pub mod ray;
pub mod raytracer;
pub mod shapes;
pub mod vec3;

pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use ray::Ray;
pub use raytracer::Raytracer;
pub use vec3::{Color, Point3, Vec3};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
