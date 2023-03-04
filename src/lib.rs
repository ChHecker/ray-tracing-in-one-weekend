pub mod camera;
pub mod hittable;
pub mod materials;
pub mod ppm;
pub mod ray;
pub mod raytracer;
pub mod shapes;
#[macro_use]
pub mod vec3;

pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use ray::Ray;
pub use raytracer::Raytracer;
pub use vec3::{Color, Point, Vec3};

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
