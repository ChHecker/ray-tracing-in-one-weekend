//! A record for when [Ray]s hit something.

use crate::materials::Material;
use crate::ray::Ray;
use crate::*;

/// A record for when a [Ray] hits something.
///
/// This struct should be returned when a [Hittable] object is hit by a [Ray] as it contains all necessary information to deal with this.
///
/// # Fields
/// - `point`: Point where the hit happened.
/// - (`u`, `v`): Coordinates on the surface submanifold (lie inside \[0,1\]).
/// - `normal`: Normal vector to the surface.
/// - `t`: Parameter of the [Ray] where the hit happened.
/// - `front_face`: Whether the hit faces the front or the back of the [Hittable].
/// - `material`: [Material] that was hit.
#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    pub point: Vector3<f32>,
    pub u: f32,
    pub v: f32,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// Create a hit record.
    pub fn new(
        point: Vector3<f32>,
        u: f32,
        v: f32,
        normal: Vector3<f32>,
        t: f32,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Self {
        HitRecord {
            point,
            u,
            v,
            normal,
            t,
            front_face,
            material,
        }
    }

    /// Create a hit record from a [Ray].
    ///
    /// This uses a [Ray] and the normal to set `front_face`.
    pub fn from_ray(
        point: Vector3<f32>,
        u: f32,
        v: f32,
        normal: Vector3<f32>,
        t: f32,
        material: &'a dyn Material,
        ray: Ray,
    ) -> Self {
        let (front_face, normal) = HitRecord::face_normal(ray, normal);
        HitRecord {
            point,
            u,
            v,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn material(&self) -> &'a dyn Material {
        self.material
    }

    /// Calculate whether the [Ray] hit the front or the back of the surface.
    fn face_normal(ray: Ray, outward_normal: Vector3<f32>) -> (bool, Vector3<f32>) {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}
