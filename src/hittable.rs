//! An abstraction over objects that can be hit by [Ray]s.
//!
//! All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).
//! This also provides a [hit record](HitRecord) for when [Ray]s hit something.

use crate::*;
use crate::{materials::Material, ray::Ray};
use rand::Rng;
use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    ops::Index,
    sync::Arc,
};

type MaterialArc = Arc<dyn Material>;
/// A record for when a [Ray] hits something.
///
/// This struct should be returned when a [Hittable] object is hit by a [Ray] as it contains all necessary information to deal with this.
///
/// # Fields
/// - `point': [Point] where the hit happened.
/// - `normal`: Normal vector to the surface.
/// - `t`: Parameter of the [Ray] where the hit happened.
/// - `front_face`: Whether the hit faces the front or the back of the [Hittable].
/// - `material`: [Material] that was hit.
#[derive(Clone, Debug)]
pub struct HitRecord {
    point: Point,
    normal: Point,
    t: f32,
    front_face: bool,
    material: MaterialArc,
}

impl HitRecord {
    /// Create a hit record.
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

    /// Create a hit record from a [Ray].
    ///
    /// This uses a [Ray] and the normal to set `front_face`.
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

    /// Calculate whether the [Ray] hit the front or the back of the surface.
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

/// An abstraction over all objects that can be hit by [Ray]s.
///
/// All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).
/// `Send + Sync` is necessary for multithreading.
pub trait Hittable: Debug + Send + Sync {
    /// Check whether a [Ray] hits the object inside a allowed parameter range.
    ///
    /// If the [Ray] does not hit the object, returns `None`. If it does, all necessary information are saved in the return [`HitRecord`].
    ///
    /// # Parameters
    /// - `ray`: [Ray] to check
    /// - `t_min`: Minimum allowed parameter of the ray (excluded).
    /// - `t_max`: Maximum allowed parameter of the ray (excluded).
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    /// Return the [`Aabb`] that completely encompasses the object.
    ///
    /// This allows for a more efficient search for hits via [bounding volume hierarchies](Bvh).
    ///
    /// # Parameters
    /// - `time0`: Start of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    /// - `time1`: End of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb>;

    /// Compare two [`Hittable`]s by the value of the `minimum` of its [`Aabb`] on an axis.
    ///
    /// This allows for sorting a list of [Hittable]s by an axis in order to create a kind of spatial hierarchy (see [Bvh]).
    ///
    /// # Parameters
    /// - `other`: Other [Hittable] to compare to.
    /// - `axis`: Axis along which the minima should be compared.
    fn cmp_box(&self, other: &dyn Hittable, axis: u8) -> Ordering {
        let box1 = self.bounding_box(0., 0.).unwrap();
        let box2 = other.bounding_box(0., 0.).unwrap();

        box1.minimum[axis]
            .partial_cmp(&box2.minimum[axis])
            .expect("NaN encountered")
    }
}

type HittableArc = Arc<dyn Hittable>;
/// Stores a list of [`Hittable`]s.
///
/// This also implements [`Hittable`] in order to be able to calulcate hits for all objects it contains, as well as calculating a [Aabb] that encompasses all objects.
///
/// # Fields
/// - `hittables`: [Vector](Vec) of [`Arc`]s of [`Hittable`]s.
#[derive(Clone, Default, Debug)]
pub struct HittableList {
    hittables: Vec<HittableArc>,
}

impl HittableList {
    /// Create an empty [`HittableList`].
    pub fn new() -> Self {
        Self {
            hittables: Vec::<HittableArc>::new(),
        }
    }

    /// Push a new [`Hittable`] to the end.
    pub fn push(&mut self, hittable: HittableArc) {
        self.hittables.push(hittable);
    }

    /// Clear the [`HittableList`].
    pub fn clear(&mut self) {
        self.hittables.clear();
    }

    /// Length of the [`HittableList`]
    pub fn len(&self) -> usize {
        self.hittables.len()
    }

    /// Remove the last [`Hittable`] and return it.
    fn pop(&mut self) -> Option<HittableArc> {
        self.hittables.pop()
    }

    /// Sort by the value of the `minimum` of the [`Aabb`]s on an axis.
    ///
    /// This allows creating a kind of spatial hierarchy (see [Bvh]).
    ///
    /// # Parameters
    /// - `axis`: Axis along which the minima should be compared.
    fn sort_by_box(&mut self, axis: u8) {
        self.hittables
            .sort_by(|a, b| Hittable::cmp_box(&**a, &**b, axis));
    }

    /// Split at `mid` and return both halves.
    fn split_at(self, mid: usize) -> (Self, Self) {
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

/// An axis-aligned bounding box.
///
/// This allows for a simple way to calculate [Ray] hits more easily by first checking for [Aabb]s encompassing the objects.
///
/// # Fields
/// - `minimum` Back bottom left [Point].
/// - `maximum` Front top right [Point].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    minimum: Point,
    maximum: Point,
}

impl Aabb {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Aabb { minimum, maximum }
    }

    /// Create an [Aabb] that encompasses two other [Aabb]s.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, hittable::*};
    /// let aabb1 = Aabb::new(
    ///     point!(-1., -1., -1.),
    ///     point!(0., 0., 0.),
    /// );
    /// let aabb2 = Aabb::new(
    ///     point!(0., 0., 0.),
    ///     point!(1., 1., 1.),
    /// );
    /// let surrounding_aabb = Aabb::surrounding(&aabb1, &aabb2);
    /// let surrounding_aabb_reference = Aabb::new(
    ///     point!(-1., -1., -1.),
    ///     point!(1., 1., 1.),
    /// );
    /// assert_eq!(surrounding_aabb, surrounding_aabb_reference);
    /// ```
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

    /// Check whether a [`Ray`] hits.
    ///
    /// See [`Hittable`] for more details on a similar function with the only difference that this only return a `bool` whether the ray hit.
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

/// Error when a [`Hittable`] cannot be encompassed by a [`Aabb`].
#[derive(Debug, Clone)]
pub struct BoundingBoxError;

impl fmt::Display for BoundingBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "encountered a Hittable that cannot have a bounding box")
    }
}

/// Bounding Volume Hierarchy.
///
/// This sorts all [`Hittable`]s into a binary tree by a random axis per level (see ['sort_by_box'](HittableList::sort_by_box)).
/// This enables a more efficient hit search (O(n log n) instead of O(n^2)) by checking the hit for the [`Aabb`] of each subtree first and than propagating down it.
///
/// # Fields
/// - `aabb`: [`Aabb`] of the subtree/node.
/// - `left`: Left subtree/node.
/// - `right`: Right subtree/node.
#[derive(Debug)]
pub struct Bvh {
    aabb: Aabb,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl Bvh {
    /// Create a new [`Bvh`] from a [`HittableList`] that will be consumed as well as a time range.
    ///
    /// This works recursively. If there is only one or two elements left in the list, they are added to the two subnodes. In all other cases, the list [is sorted by a random axis](HittableList::sort_by_box), split in half, and propagated down.
    ///
    /// # Parameters
    /// - `hittables`: [`HittableList`] to sort into the tree (consumed).
    /// - `time0`: Starting time.
    /// - `time1`: Ending time.
    pub fn new(
        mut hittables: HittableList,
        time0: f32,
        time1: f32,
    ) -> Result<Self, BoundingBoxError> {
        for hittable in &hittables.hittables {
            if let None = hittable.bounding_box(0., 0.) {
                return Err(BoundingBoxError);
            }
        }

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

            let mid = hittables.len() / 2;
            let split = hittables.split_at(mid);

            left = Arc::new(Bvh::new(split.0, time0, time1)?);
            right = Arc::new(Bvh::new(split.1, time0, time1)?);
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

/// Options to store [`Hittable`]s.
///
/// Both [`HittableList`] and [`Bvh`] can store [`Hittable`]s. Latter is faster, but not always possible (see [`BoundingBoxError`], e.g. an infinite plane).
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
