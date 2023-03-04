use std::sync::Arc;

use crate::{materials::Material, *};

type MaterialArc = Arc<dyn Material>;
pub struct HitRecord {
    point: Point,
    normal: Point,
    t: f32,
    front_face: bool,
    material: MaterialArc,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Point,
        t: f32,
        front_face: bool,
        material: MaterialArc,
    ) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn from_ray(point: Point, normal: Point, t: f32, material: MaterialArc, ray: Ray) -> Self {
        let (front_face, normal) = HitRecord::face_normal(ray, normal);
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn normal(&self) -> Point {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> MaterialArc {
        self.material.clone()
    }

    fn face_normal(ray: Ray, outward_normal: Point) -> (bool, Point) {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

type HittableArc = Arc<dyn Hittable>;
pub struct HittableList {
    hittables: Vec<HittableArc>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittables: Vec::<HittableArc>::new(),
        }
    }

    pub fn push(&mut self, hittable: HittableArc) {
        self.hittables.push(hittable);
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record_final: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                hit_record_final = Some(hit_record);
            }
        }

        hit_record_final
    }
}
