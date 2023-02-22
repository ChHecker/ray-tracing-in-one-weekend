use crate::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<(Ray, Color)>;
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
    fn scatter(&self, _ray: &Ray, hit: HitRecord) -> Option<(Ray, Color)> {
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
    fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<(Ray, Color)> {
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
