use rand::Rng;

use crate::*;

pub trait Material {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal() + Point3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal();
        }

        let scattered = Ray::new(hit.point(), scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1. { fuzz } else { 1. };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit.normal());
        let scattered = Ray::new(
            hit.point(),
            reflected + self.fuzz * Point3::random_in_unit_sphere(),
        );
        if scattered.direction().dot(&hit.normal()) > 0. {
            return Some((scattered, self.albedo));
        }
        None
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cos: f64, refraction_ratio: f64) -> f64 {
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
        let cos_theta = f64::min(-unit_direction.dot(&hit.normal()), 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refrect = refraction_ratio * sin_theta > 1.;
        let direction: Point3;

        if cannot_refrect || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
            direction = unit_direction.reflect(&hit.normal());
        } else {
            direction = unit_direction.refract(&hit.normal(), refraction_ratio);
        }

        let scattered = Ray::new(hit.point(), direction);
        Some((scattered, Color::new(1., 1., 1.)))
    }
}
