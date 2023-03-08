use crate::{materials::Material, *};
use rand::Rng;
use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    ops::Index,
    sync::Arc,
};

type MaterialArc = Arc<dyn Material>;
#[derive(Clone, Debug)]
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

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb>;

    fn cmp_box(&self, other: &dyn Hittable, axis: u8) -> Ordering {
        let box1 = self.bounding_box(0., 0.).unwrap();
        let box2 = other.bounding_box(0., 0.).unwrap();

        box1.minimum[axis]
            .partial_cmp(&box2.minimum[axis])
            .expect("NaN encountered")
    }
}

type HittableArc = Arc<dyn Hittable>;
#[derive(Clone, Default, Debug)]
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

    pub fn len(&self) -> usize {
        self.hittables.len()
    }

    fn pop(&mut self) -> Option<HittableArc> {
        self.hittables.pop()
    }

    fn sort_by_box(&mut self, axis: u8) {
        self.hittables
            .sort_by(|a, b| Hittable::cmp_box(&**a, &**b, axis));
    }

    fn split_at_half(self) -> (Self, Self) {
        let mid = self.len() / 2;
        let (left, right) = self.hittables.split_at(mid);
        (
            Self {
                hittables: left.to_vec(),
            },
            Self {
                hittables: right.to_vec(),
            },
        )
    }
}

impl Index<usize> for HittableList {
    type Output = HittableArc;

    fn index(&self, index: usize) -> &Self::Output {
        &self.hittables[index]
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
                    Some(aabb) => aabb_out = Some(Aabb::surrounding(&aabb, &aabb_hit)),
                },
                None => return None,
            }
        }

        aabb_out
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    minimum: Point,
    maximum: Point,
}

impl Aabb {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Aabb { minimum, maximum }
    }

    pub fn surrounding(&self, aabb: &Self) -> Self {
        let minimum = point![
            f32::min(self.minimum().x(), aabb.minimum().x()),
            f32::min(self.minimum().y(), aabb.minimum().y()),
            f32::min(self.minimum().z(), aabb.minimum().z()),
        ];
        let maximum = point![
            f32::max(self.maximum().x(), aabb.maximum().x()),
            f32::max(self.maximum().y(), aabb.maximum().y()),
            f32::max(self.maximum().z(), aabb.maximum().z()),
        ];
        Aabb { minimum, maximum }
    }

    pub fn minimum(&self) -> Point {
        self.minimum
    }

    pub fn maximum(&self) -> Point {
        self.maximum
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
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

#[derive(Debug, Clone)]
pub struct BoundingBoxError;

impl fmt::Display for BoundingBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "encountered a Hittable that cannot have a bounding box")
    }
}

#[derive(Debug)]
pub struct Bvh {
    aabb: Aabb,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl Bvh {
    pub fn new(hittables: HittableList, time0: f32, time1: f32) -> Result<Self, BoundingBoxError> {
        for hittable in &hittables.hittables {
            if let None = hittable.bounding_box(0., 0.) {
                return Err(BoundingBoxError);
            }
        }
        Self::new_recursive(hittables, time0, time1)
    }

    fn new_recursive(
        mut hittables: HittableList,
        time0: f32,
        time1: f32,
    ) -> Result<Self, BoundingBoxError> {
        let mut rand = rand::thread_rng();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>);
        let axis: u8 = rand.gen_range(0..=2);

        if hittables.len() == 1 {
            let elem = hittables.pop().unwrap();
            left = elem.clone();
            right = elem;
        } else if hittables.len() == 2 {
            let last = hittables.pop().unwrap();
            let first = hittables.pop().unwrap();
            match first.cmp_box(&*last, axis) {
                Ordering::Less | Ordering::Equal => {
                    left = first;
                    right = last;
                }
                Ordering::Greater => {
                    left = last;
                    right = first;
                }
            }
        } else {
            hittables.sort_by_box(axis);

            let split = hittables.split_at_half();

            left = Arc::new(Bvh::new_recursive(split.0, time0, time1)?);
            right = Arc::new(Bvh::new_recursive(split.1, time0, time1)?);
        }

        let aabb = Aabb::surrounding(
            &left.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
            &right.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
        );

        Ok(Self { aabb, left, right })
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let t_max = match &hit_left {
            Some(hit_record) => hit_record.t,
            None => t_max,
        };
        let hit_right = self.right.hit(ray, t_min, t_max);

        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }
}

pub enum HittableListOptions<'a> {
    HittableList(&'a HittableList),
    Bvh(&'a Bvh),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::materials::Lambertian;
    use crate::shapes::Sphere;

    #[test]
    fn surrounding_aabb() {
        let aabb1 = Aabb {
            minimum: point!(-1., -1., 0.),
            maximum: point!(0., 0., 0.),
        };
        let aabb2 = Aabb {
            minimum: point!(0., 0., 0.),
            maximum: point!(1., 1., 0.),
        };
        let surrounding_aabb = Aabb::surrounding(&aabb1, &aabb2);
        let surrounding_aabb_reference = Aabb {
            minimum: point!(-1., -1., 0.),
            maximum: point!(1., 1., 0.),
        };
        assert_eq!(surrounding_aabb, surrounding_aabb_reference);
    }

    #[test]
    fn bvh_hit() {
        let left = Arc::new(Sphere::new(
            point![-2., 0., -1.],
            1.,
            Arc::new(Lambertian::new(color![1., 1., 1.])),
        ));
        let right = Arc::new(Sphere::new(
            point![2., 0., -1.],
            1.,
            Arc::new(Lambertian::new(color![1., 1., 1.])),
        ));
        let aabb = Aabb::surrounding(
            &left.bounding_box(0., 0.).unwrap(),
            &right.bounding_box(0., 0.).unwrap(),
        );
        let bvh = Bvh { aabb, left, right };

        let ray_hit_left = Ray::new(point![0., 0., 0.], point![-2., 0., -1.]);
        let hit_left = bvh.hit(ray_hit_left, 0., f32::INFINITY);
        assert_eq!(hit_left.is_some(), true);

        let ray_hit_right = Ray::new(point![0., 0., 0.], point![2., 0., -1.]);
        let hit_right = bvh.hit(ray_hit_right, 0., f32::INFINITY);
        assert_eq!(hit_right.is_some(), true);

        let ray_no_hit = Ray::new(point![0., 0., 0.], point![0., 0., 1.]);
        let no_hit = bvh.hit(ray_no_hit, 0., f32::INFINITY);
        assert_eq!(no_hit.is_some(), false);
    }
}
