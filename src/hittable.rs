use crate::{Point3, Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    point: Point3,
    normal: Point3,
    t: f64,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Point3, t: f64) -> Self {
        HitRecord { point, normal, t }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Point3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
