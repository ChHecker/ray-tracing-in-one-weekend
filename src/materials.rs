//! Collection of materials of [`Hittable`]s.

use std::fmt::Debug;

use rand::Rng;

use crate::ray::Ray;
use crate::textures::Texture;
use crate::*;

/// An abstraction for materials of [`Hittable`]s.
///
/// Each materials reemits one [`Ray`] in some way and also includes its color when hit by one.
pub trait Material: Debug + Send + Sync {
    /// Scatters the incoming [`Ray`] into an outgoing [`Ray`] and includes [`Color`] information.
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)>;
}

/// A realistic perfectly diffusive material.
///
/// # Fields
/// - `albedo`: Color of the [`Lambertian`].
#[derive(Clone, Debug)]
pub struct Lambertian<'a, T: Texture> {
    albedo: &'a T,
}

impl<'a, T: Texture> Lambertian<'a, T> {
    pub fn new(albedo: &'a T) -> Self {
        Self { albedo }
    }
}

impl<'a, T: Texture> Material for Lambertian<'a, T> {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal() + Point::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal();
        }

        let scattered = Ray::new(hit.point(), scatter_direction).with_time(ray.time());
        Some((
            scattered,
            self.albedo.color_at(hit.u(), hit.v(), hit.point()),
        ))
    }
}

/// A fuzzy reflective material (metal).
#[derive(Clone, Debug)]
pub struct Metal<'a, T: Texture> {
    albedo: &'a T,
    fuzz: f32,
}

impl<'a, T: Texture> Metal<'a, T> {
    pub fn new(albedo: &'a T, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1. { fuzz } else { 1. };
        Self { albedo, fuzz }
    }
}

impl<'a, T: Texture> Material for Metal<'a, T> {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit.normal());
        let scattered = Ray::new(
            hit.point(),
            reflected + self.fuzz * Point::random_in_unit_sphere(),
        )
        .with_time(ray.time());
        if scattered.direction().dot(&hit.normal()) > 0. {
            return Some((
                scattered,
                self.albedo.color_at(hit.u(), hit.v(), hit.point()),
            ));
        }
        None
    }
}

/// A transparent material.
#[derive(Clone, Debug)]
pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cos: f32, refraction_ratio: f32) -> f32 {
        let mut r0 = (1. - refraction_ratio) / (1. + refraction_ratio);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();

        let refraction_ratio = if hit.front_face() {
            1. / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction().unit_vector();
        let cos_theta = f32::min(-unit_direction.dot(&hit.normal()), 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refrect = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_refrect || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(&hit.normal())
            } else {
                unit_direction.refract(&hit.normal(), refraction_ratio)
            };

        let scattered = Ray::new(hit.point(), direction).with_time(ray.time());
        Some((scattered, color![1., 1., 1.]))
    }
}
