use crate::{Point3, Ray, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    point: Point3,
    normal: Point3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Point3, t: f64, front_face: bool) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }

    pub fn from_ray(point: Point3, normal: Point3, t: f64, ray: &Ray) -> Self {
        let (front_face, normal) = HitRecord::face_normal(ray, normal);
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
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

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    fn face_normal(ray: &Ray, outward_normal: Point3) -> (bool, Point3) {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
