//! A simple ray tracer following Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io) book series.
//!
//! As the coordinate system is arbitrary, it was chosen in this way:
//!  - y axis points up
//!  - z axis points towards the camera
//! In order to create a ray-traced image, one needs to create a [`Camera`], then a [`Raytracer`] and add [`Hittable`]s to its `world`.

pub mod camera;
pub mod hitrecord;
pub mod hittable;
pub mod materials;
pub mod ppm;
pub mod ray;
pub mod raytracer;
pub mod shapes;
pub mod textures;
#[macro_use]
pub mod vec3;

pub use camera::Camera;
pub use hittable::{Hittable, HittableList};
pub use raytracer::Raytracer;
pub use vec3::{Color, Point, Vec3};
