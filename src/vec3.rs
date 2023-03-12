//! Utility functions on [`Vector3`].

use nalgebra::Vector3;
use rand::Rng;

/// Reflects the vector.
///
/// The reflection follows the rule of equal angles with respect to `normal`.
///
/// # Example
/// ```
/// # use ray_tracing_in_one_weekend::{*, vec3::*};
/// let v = vector![1., 2., 3.].normalize();
/// let normal = v;
/// assert!((reflect(&v, &normal) + v).norm() < 0.01);
/// ```
pub fn reflect(vec: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    *vec - 2. * vec.dot(normal) * *normal
}

/// Refracts the vector.
///
/// The refraction follows Snell's law with the fraction of refraction indices `etai_over_etat`.
///
/// # Example
/// ```
/// # use ray_tracing_in_one_weekend::{*, vec3::*};
/// let v = vector![1., 2., 3.].normalize();
/// let normal = v;
/// assert!((refract(&v, &normal, 1.) + normal).norm() < 0.01);
/// ```
pub fn refract(vec: &Vector3<f32>, normal: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min(-vec.dot(normal), 1.);
    let refracted_out_perp = etai_over_etat * (*vec + cos_theta * *normal);
    let refracted_out_parallel = -(1. - refracted_out_perp.norm_squared()).abs().sqrt() * *normal;
    refracted_out_perp + refracted_out_parallel
}

/// Checks whether a vector is near 0 (up to 1e-8) in every component.
///
/// # Example
/// ```
/// # use ray_tracing_in_one_weekend::{*, vec3::*};
/// let v = vector![1e-9, 1e-9, 1e-9];
/// assert_eq!(near_zero(&v), true);
/// ```
pub fn near_zero(vec: &Vector3<f32>) -> bool {
    let s = 1e-8;
    vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s
}

/// Creates a random vector with each element between 0 and 1.
pub fn random_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

/// Creates a random vector with each element in a range.
pub fn random_vector_in_range(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        min + rng.gen::<f32>() * (max - min),
        min + rng.gen::<f32>() * (max - min),
        min + rng.gen::<f32>() * (max - min),
    )
}

pub fn random_vector_in_unit_sphere() -> Vector3<f32> {
    loop {
        let rand = random_vector_in_range(-1., 1.);
        if rand.norm_squared() < 1. {
            return rand;
        }
    }
}

pub fn random_unit_vector_in_unit_sphere() -> Vector3<f32> {
    random_vector_in_unit_sphere().normalize()
}

pub fn random_vector_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let rand = random_vector_in_unit_sphere();
    if rand.dot(normal) > 0. {
        return rand;
    }
    -rand
}

pub fn random_vector_in_unit_disk() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let rand = Vector3::new(-1. + rng.gen::<f32>() * 2., -1. + rng.gen::<f32>() * 2., 0.);
        if rand.norm_squared() < 1. {
            return rand;
        }
    }
}
