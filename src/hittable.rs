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
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb>;
}

type HittableArc = Arc<dyn Hittable>;
#[derive(Clone, Default)]
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        if self.hittables.is_empty() {
            return None;
        }

        let mut aabb_out: Option<Aabb> = None;

        for hittable in &self.hittables {
            match hittable.bounding_box(time0, time1) {
                Some(aabb_hit) => match aabb_out {
                    None => aabb_out = Some(aabb_hit),
                    Some(aabb) => aabb_out = Some(Aabb::surrounding_aabb(&aabb, &aabb_hit)),
                },
                None => return None,
            }
        }

        aabb_out
    }
}

pub struct Aabb {
    minimum: Point,
    maximum: Point,
}

impl Aabb {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Aabb { minimum, maximum }
    }

    pub fn surrounding_aabb(&self, aabb: &Self) -> Self {
        let minimum = point![
            f32::min(self.minimum().x(), aabb.minimum().x()),
            f32::min(self.minimum().y(), aabb.minimum().y()),
            f32::min(self.minimum().z(), aabb.minimum().z()),
        ];
        let maximum = point![
            f32::min(self.maximum().x(), aabb.maximum().x()),
            f32::min(self.maximum().y(), aabb.maximum().y()),
            f32::min(self.maximum().z(), aabb.maximum().z()),
        ];
        Aabb { minimum, maximum }
    }

    pub fn minimum(&self) -> Point {
        self.minimum
    }

    pub fn maximum(&self) -> Point {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for (((min, max), ray_direction), ray_origin) in self
            .minimum()
            .into_iter()
            .zip(self.maximum().into_iter())
            .zip(ray.direction().into_iter())
            .zip(ray.origin().into_iter())
        {
            let inverse_distance = 1. / ray_direction;
            let mut t0 = (min - ray_origin) * inverse_distance;
            let mut t1 = (max - ray_origin) * inverse_distance;
            if inverse_distance < 0. {
                (t0, t1) = (t1, t0);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
