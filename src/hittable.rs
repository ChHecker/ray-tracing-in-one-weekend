//! An abstraction over objects that can be hit by [Ray]s.
//!
//! All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).

use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::ops::Index;

use rand::Rng;

use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::*;

type HittableBox = Box<dyn Hittable>;

/// Allows for cloning [`Hittable`]s.
pub trait HittableClone {
    fn box_clone(&self) -> HittableBox;
}

impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn box_clone(&self) -> HittableBox {
        Box::new(self.clone())
    }
}

impl Clone for HittableBox {
    fn clone(&self) -> HittableBox {
        self.box_clone()
    }
}

/// An abstraction over all objects that can be hit by [Ray]s.
///
/// All objects that can be hit by [`Ray`]s and encompassed by [axis-aligned bounding boxes](Aabb) should implement [`Hittable`]. This not only includes shapes, but also more abstract objects like [lists of shapes](HittableList).
/// `Send + Sync` is necessary for multithreading.
pub trait Hittable: Debug + HittableClone + Send + Sync {
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

/// Stores a list of [`Hittable`]s.
///
/// # Fields
/// - `hittables`: [Vector](Vec) of [`Arc`]s of [`Hittable`]s.
#[derive(Clone, Default, Debug)]
pub struct HittableList {
    hittables: Vec<HittableBox>,
}

impl HittableList {
    /// Create an empty [`HittableList`].
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    /// Push a new [`Hittable`] to the end.
    pub fn push<H: Hittable + 'static>(&mut self, hittable: H) {
        self.hittables.push(Box::new(hittable));
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

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
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

    /// Remove the last [`Hittable`] and return it.
    fn pop(&mut self) -> Option<HittableBox> {
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
        let mut left = Vec::<HittableBox>::new();
        let mut right = Vec::<HittableBox>::new();
        for (i, hittable) in self.hittables.into_iter().enumerate() {
            if i < mid {
                left.push(hittable);
            } else {
                right.push(hittable);
            }
        }
        (Self { hittables: left }, Self { hittables: right })
    }
}

impl Index<usize> for HittableList {
    type Output = HittableBox;

    fn index(&self, index: usize) -> &Self::Output {
        &self.hittables[index]
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

/// Possible nodes in a [`Bvh`].
///
/// [`Bvh`]s are binary trees and might therefore sometimes end with only one node. With this enum, [`Option`] is not needed.
#[derive(Clone, Debug)]
enum BvhNode {
    One(HittableBox),
    Two(HittableBox, HittableBox),
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
        Bvh::check_hittable_list(&hittables)?;

        let mut rand = rand::thread_rng();

        let subnode: BvhNode;
        let axis: u8 = rand.gen_range(0..=2);

        if hittables.len() == 1 {
            let elem = hittables.pop().unwrap();
            subnode = BvhNode::One(elem);
        } else if hittables.len() == 2 {
            let last = hittables.pop().unwrap();
            let first = hittables.pop().unwrap();
            match first.cmp_box(&*last, axis) {
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

            let left = Box::new(Bvh::new(split.0, time0, time1)?);
            let right = Box::new(Bvh::new(split.1, time0, time1)?);

            subnode = BvhNode::Two(left, right);
        }

        let aabb = match &subnode {
            BvhNode::One(child) => child.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
            BvhNode::Two(left, right) => Aabb::surrounding(
                &left.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
                &right.bounding_box(time0, time1).ok_or(BoundingBoxError)?,
            ),
        };

        Ok(Self { aabb, subnode })
    }

    pub fn check_hittable_list(hittables: &HittableList) -> Result<(), BoundingBoxError> {
        if hittables.is_empty() {
            return Err(BoundingBoxError);
        }

        for hittable in &hittables.hittables {
            if hittable.bounding_box(0., 0.).is_none() {
                return Err(BoundingBoxError);
            }
        }

        Ok(())
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match &self.subnode {
            BvhNode::One(child) => child.hit(ray, t_min, t_max),
            BvhNode::Two(left, right) => {
                let hit_left = left.hit(ray, t_min, t_max);
                let t_max = match &hit_left {
                    Some(hit_record) => hit_record.t(),
                    None => t_max,
                };
                let hit_right = right.hit(ray, t_min, t_max);

                hit_right.or(hit_left)
            }
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.aabb)
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
        let left = Box::new(Sphere::new(
            point![-2., 0., -1.],
            1.,
            black_lambertian.clone(),
        ));
        let right = Box::new(Sphere::new(point![2., 0., -1.], 1., black_lambertian));
        let aabb = Aabb::surrounding(
            &left.bounding_box(0., 0.).unwrap(),
            &right.bounding_box(0., 0.).unwrap(),
        );
        let subnode = BvhNode::Two(left, right);
        let bvh = Bvh { aabb, subnode };

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
