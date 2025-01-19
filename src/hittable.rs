//! An abstraction over objects that can be hit by [Ray]s.
//!
//! All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).

use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::ops::{Deref, Index};
use std::sync::Arc;

use nalgebra::Rotation3;
use rand::Rng;

use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::shapes::{Movable, Offset};
use crate::*;

type HittableArc = Arc<dyn Hittable>;

/// An abstraction over all objects that can be hit by [Ray]s.
///
/// All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).
/// `Send + Sync` is necessary for multithreading.
pub trait Hittable: Debug + Send + Sync {
    /// Check whether a [Ray] hits the object at the origin inside an allowed parameter range.
    ///
    /// **Do not manually use this function! This should only be overwritten for new [`shapes`], but not manually used! Use [`hit`](Hittable::hit) instead!**
    ///
    /// # Parameters
    /// - `ray`: [Ray] to check
    /// - `t_min`: Minimum allowed parameter of the ray (excluded).
    /// - `t_max`: Maximum allowed parameter of the ray (excluded).
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    /// Return the [`Aabb`] that completely encompasses the object at the origin.
    ///
    /// **Do not manually use this function! This should only be overwritten for new [`shapes`], but not manually used! Use [`bounding_box`](Hittable::bounding_box) instead!**
    ///
    /// # Parameters
    /// - `time0`: Start of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    /// - `time1`: End of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    fn bounding_box_origin(&self, time0: f32, time1: f32) -> Option<Aabb>;

    /// Check whether a [Ray] hits the object inside an allowed parameter range.
    ///
    /// If the [Ray] does not hit the object, returns `None`. If it does, all necessary information are saved in the return [`HitRecord`].
    /// This function calls [`Offset::hit`], which in turn calls [`Hittable::hit_origin`] with an offset [`Ray`].
    ///
    /// # Parameters
    /// - `ray`: [Ray] to check
    /// - `t_min`: Minimum allowed parameter of the ray (excluded).
    /// - `t_max`: Maximum allowed parameter of the ray (excluded).
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.center().hit(self, ray, t_min, t_max)
    }

    /// Return the [`Aabb`] that completely encompasses the object at the origin.
    ///
    /// This allows for a more efficient search for hits via [bounding volume hierarchies](Bvh).
    ///
    /// # Parameters
    /// - `time0`: Start of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    /// - `time1`: End of the interval in which the object should be fully encompassed. Set to `0.` if no time resolution is desired.
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        self.center().bounding_box(self, time0, time1)
    }

    /// Compare two [`Hittable`]s by the value of the `minimum` of its [`Aabb`] on an axis.
    ///
    /// This allows for sorting a list of [Hittable]s by an axis in order to create a kind of spatial hierarchy (see [Bvh]).
    ///
    /// # Parameters
    /// - `other`: Other [Hittable] to compare to.
    /// - `axis`: Axis along which the minima should be compared.
    fn cmp_box(&self, other: &dyn Hittable, axis: usize) -> Ordering {
        let box1 = self.bounding_box(0., 0.).unwrap();
        let box2 = other.bounding_box(0., 0.).unwrap();

        box1.minimum[axis]
            .partial_cmp(&box2.minimum[axis])
            .expect("NaN encountered")
    }

    /// Get a reference to the center ([`Offset`]) of the [`Hittable`].
    fn center(&self) -> &Offset;
}

/// Stores a list of [`Hittable`]s.
///
/// # Fields
/// - `hittables`: [Vector](Vec) of [`Box`]s of [`Hittable`]s.
#[derive(Clone, Default, Debug)]
pub struct HittableList {
    center: Offset,
    hittables: Vec<HittableArc>,
}

impl HittableList {
    /// Create an empty [`HittableList`].
    pub fn new(center: Vector3<f32>) -> Self {
        Self {
            center: Offset::new(center),
            hittables: Vec::new(),
        }
    }

    /// Push a new [`Hittable`] to the end.
    pub fn push<H: Hittable + 'static>(&mut self, hittable: H) {
        self.hittables.push(Arc::new(hittable));
    }

    /// Clear the [`HittableList`].
    pub fn clear(&mut self) {
        self.hittables.clear();
    }

    /// Whether the [`HittableList`] is empty.
    pub fn is_empty(&self) -> bool {
        self.hittables.is_empty()
    }

    /// Length of the [`HittableList`].
    pub fn len(&self) -> usize {
        self.hittables.len()
    }

    /// Remove the last [`Hittable`] and return it.
    pub fn pop(&mut self) -> Option<HittableArc> {
        self.hittables.pop()
    }

    /// Sort by the value of the `minimum` of the [`Aabb`]s on an axis.
    ///
    /// This allows creating a kind of spatial hierarchy (see [Bvh]).
    ///
    /// # Parameters
    /// - `axis`: Axis along which the minima should be compared.
    fn sort_by_box(&mut self, axis: usize) {
        self.hittables
            .sort_by(|a, b| Hittable::cmp_box(a.deref(), b.deref(), axis));
    }

    /// Split at `mid` and return both halves.
    fn split_at(self, mid: usize) -> (Self, Self) {
        let (left, right) = self.hittables.split_at(mid);

        (
            Self {
                hittables: left.to_owned(),
                center: self.center.clone(),
            },
            Self {
                hittables: right.to_owned(),
                center: self.center,
            },
        )
    }
}

impl Hittable for HittableList {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record_final: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_record_final = Some(hit_record);
            }
        }

        hit_record_final
    }

    fn bounding_box_origin(&self, time0: f32, time1: f32) -> Option<Aabb> {
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

    fn center(&self) -> &Offset {
        &self.center
    }
}

impl Movable for HittableList {
    fn with_rotation(mut self, rotation: Rotation3<f32>) -> Self {
        self.center = self.center.with_rotation(rotation);
        self
    }

    fn moving(mut self, offset_end: Vector3<f32>, time_start: f32, time_end: f32) -> Self {
        self.center = self.center.moving(offset_end, time_start, time_end);
        self
    }
}

impl Index<usize> for HittableList {
    type Output = HittableArc;

    fn index(&self, index: usize) -> &Self::Output {
        &self.hittables[index]
    }
}

/// An axis-aligned bounding box.
///
/// This allows for a simple way to calculate [Ray] hits more easily by first checking for [Aabb]s encompassing the objects.
///
/// # Fields
/// - `minimum` Back bottom left point.
/// - `maximum` Front top right point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub minimum: Vector3<f32>,
    pub maximum: Vector3<f32>,
}

impl Aabb {
    pub fn new(minimum: Vector3<f32>, maximum: Vector3<f32>) -> Self {
        Aabb { minimum, maximum }
    }

    /// Create an [Aabb] that encompasses two other [Aabb]s.
    ///
    /// # Example
    /// ```
    /// # use ray_tracing_in_one_weekend::{*, hittable::*};
    /// let aabb1 = Aabb::new(
    ///     vector!(-1., -1., -1.),
    ///     vector!(0., 0., 0.),
    /// );
    /// let aabb2 = Aabb::new(
    ///     vector!(0., 0., 0.),
    ///     vector!(1., 1., 1.),
    /// );
    /// let surrounding_aabb = Aabb::surrounding(&aabb1, &aabb2);
    /// let surrounding_aabb_reference = Aabb::new(
    ///     vector!(-1., -1., -1.),
    ///     vector!(1., 1., 1.),
    /// );
    /// assert_eq!(surrounding_aabb, surrounding_aabb_reference);
    /// ```
    pub fn surrounding(&self, aabb: &Self) -> Self {
        let minimum = vector![
            f32::min(self.minimum().x, aabb.minimum().x),
            f32::min(self.minimum().y, aabb.minimum().y),
            f32::min(self.minimum().z, aabb.minimum().z)
        ];
        let maximum = vector![
            f32::max(self.maximum().x, aabb.maximum().x),
            f32::max(self.maximum().y, aabb.maximum().y),
            f32::max(self.maximum().z, aabb.maximum().z)
        ];
        Aabb { minimum, maximum }
    }

    pub fn minimum(&self) -> Vector3<f32> {
        self.minimum
    }

    pub fn maximum(&self) -> Vector3<f32> {
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

/// Possible nodes in a [`Bvh`].
///
/// [`Bvh`]s are binary trees and might therefore sometimes end with only one node. With this enum, [`Option`] is not needed.
#[derive(Clone, Debug)]
enum BvhNode {
    One(HittableArc),
    Two(HittableArc, HittableArc),
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
#[derive(Clone, Debug)]
pub(crate) struct Bvh {
    center: Offset,
    aabb: Aabb,
    subnode: BvhNode,
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
        if !Bvh::check_hittable_list(&hittables) {
            return Err(BoundingBoxError);
        }

        let mut rand = rand::thread_rng();

        let center = hittables.center.clone();
        let subnode: BvhNode;
        let axis: usize = rand.gen_range(0..=2);

        if hittables.len() == 1 {
            let elem = hittables.pop().unwrap();
            subnode = BvhNode::One(elem);
        } else if hittables.len() == 2 {
            let last = hittables.pop().unwrap();
            let first = hittables.pop().unwrap();
            match first.cmp_box(last.deref(), axis) {
                Ordering::Less | Ordering::Equal => {
                    subnode = BvhNode::Two(first, last);
                }
                Ordering::Greater => {
                    subnode = BvhNode::Two(last, first);
                }
            }
        } else {
            hittables.sort_by_box(axis);

            let mid = hittables.len() / 2;
            let split = hittables.split_at(mid);

            let left = Arc::new(Bvh::new(split.0, time0, time1)?);
            let right = Arc::new(Bvh::new(split.1, time0, time1)?);

            subnode = BvhNode::Two(left, right);
        }

        let aabb = match &subnode {
            BvhNode::One(child) => child.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
            BvhNode::Two(left, right) => Aabb::surrounding(
                &left.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
                &right.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
            ),
        };

        Ok(Self {
            center,
            aabb,
            subnode,
        })
    }

    pub fn check_hittable_list(hittables: &HittableList) -> bool {
        if hittables.is_empty() {
            return false;
        }

        for hittable in &hittables.hittables {
            if hittable.bounding_box(0., 0.).is_none() {
                return false;
            }
        }

        true
    }
}

impl Hittable for Bvh {
    fn hit_origin(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match &self.subnode {
            BvhNode::One(child) => child.hit(ray, t_min, t_max),
            BvhNode::Two(left, right) => {
                let hit_left = left.hit(ray, t_min, t_max);
                let t_max = match &hit_left {
                    Some(hit_record) => hit_record.t,
                    None => t_max,
                };
                let hit_right = right.hit(ray, t_min, t_max);

                hit_right.or(hit_left)
            }
        }
    }

    fn bounding_box_origin(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }

    fn center(&self) -> &Offset {
        &self.center
    }
}

/// Options to store [`Hittable`]s.
///
/// Both [`HittableList`] and [`Bvh`] can store [`Hittable`]s. Latter is faster, but not always possible (see [`BoundingBoxError`], e.g. an infinite plane).
pub(crate) enum HittableListOptions {
    HittableList(HittableList),
    Bvh(Bvh),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::materials::Lambertian;
    use crate::shapes::Sphere;
    use crate::textures::SolidColor;

    #[test]
    fn bvh_hit() {
        let black = SolidColor::new(color![1., 1., 1.]);
        let black_lambertian = Lambertian::new(black);
        let left = Arc::new(Sphere::new(
            vector![-2., 0., -1.],
            1.,
            black_lambertian.clone(),
        ));
        let right = Arc::new(Sphere::new(vector![2., 0., -1.], 1., black_lambertian));
        let aabb = Aabb::surrounding(
            &left.bounding_box(0., 0.).unwrap(),
            &right.bounding_box(0., 0.).unwrap(),
        );
        let subnode = BvhNode::Two(left, right);
        let bvh = Bvh {
            aabb,
            subnode,
            center: Offset::default(),
        };

        let ray_hit_left = Ray::new(vector![0., 0., 0.], vector![-2., 0., -1.]);
        let hit_left = bvh.hit(ray_hit_left, 0., f32::INFINITY);
        assert!(hit_left.is_some());

        let ray_hit_right = Ray::new(vector![0., 0., 0.], vector![2., 0., -1.]);
        let hit_right = bvh.hit(ray_hit_right, 0., f32::INFINITY);
        assert!(hit_right.is_some());

        let ray_no_hit = Ray::new(vector![0., 0., 0.], vector![0., 0., 1.]);
        let no_hit = bvh.hit(ray_no_hit, 0., f32::INFINITY);
        assert!(no_hit.is_none());
    }
}
