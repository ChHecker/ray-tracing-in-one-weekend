//! A camera that receives [`Ray`]s.

use rand::Rng;

use crate::ray::Ray;
use crate::vec3::random_in_unit_disk;
use crate::*;

/// A struct for a camera.
///
/// This stores all necessary information about the viewport as well as the depth-of-field.
///
/// # Fields
/// - `origin`: Point where the camera is positioned.
/// - `lower_left_corner`: Lower left corner of the viewport.
/// - `horizontal`: Horizontal stretch of the viewport.
/// - `vertical`: Vertical stretch of the viewport.
/// - `u`: Normal to the direction the camera is facing and its upwards direction.
/// - `v`: Unit upwards direction.
/// - `w`: Unit direction the camera is facing.
/// - `lens_radius` Radius of the lense for the purpose of depth-of-field (half the aperture).
/// - `time`: Optional exposure time.
#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    _w: Vector3<f32>,
    lens_radius: f32,
    time: Option<(f32, f32)>,
}

impl Camera {
    /// Create a new camera.
    ///
    /// # Parameters
    /// - `lookfrom`: Position of the camera.
    /// - `lookat`: [Vector3<f32>] the camera is facing.
    /// - `vup`: Upwards direction of the camera.
    /// - `vertical_fov`: Angle of the vertical field of view (between point the camera is facing and the upper border of the viewport).
    /// - `aspect_ratio`: Aspect ratio of the viewport.
    /// - `aperture`: Aperture for the purpose of depth-of-field (double the radius of the lense).
    /// - `focus_distance`: Distance at which objects appear in focus.
    pub fn new(
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let h = (vertical_fov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2. - vertical / 2. - focus_distance * w;

        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius: aperture / 2.,
            time: None,
        }
    }

    /// Consume `self` and create a [`Camera`] with a non-zero exposure.
    pub fn with_time(mut self, time_start: f32, time_end: f32) -> Self {
        self.time = Some((time_start, time_end));
        self
    }

    /// Emit a [`Ray`] from the camera.
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let mut rng = rand::thread_rng();

        let random_disk = self.lens_radius * random_in_unit_disk();
        let offset = self.u * random_disk.x + self.v * random_disk.y;

        let ray = Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        );
        if let Some((time1, time2)) = self.time {
            ray.with_time(time1 + rng.gen::<f32>() * (time2 - time1))
        } else {
            ray
        }
    }

    pub fn time(&self) -> Option<(f32, f32)> {
        self.time
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            vector![0., 0., 0.],
            vector![0., 0., -1.],
            vector![0., 1., 0.],
            std::f32::consts::FRAC_PI_6,
            16. / 9.,
            0.,
            1.,
        )
    }
}
