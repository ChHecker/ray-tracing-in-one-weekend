use rand::Rng;

use crate::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    u: Point3,
    v: Point3,
    w: Point3,
    lens_radius: f64,
    time1: f64,
    time2: f64,
}

impl Camera {
    // FOV: Radians
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Point3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        time1: f64,
        time2: f64,
    ) -> Self {
        let h = (vertical_fov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
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
            w,
            lens_radius: aperture / 2.,
            time1,
            time2,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let random_disk = self.lens_radius * Point3::random_in_unit_disk();
        let offset = self.u * random_disk.x() + self.v * random_disk.y();

        Ray::new_with_time(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            self.time1 + rng.gen::<f64>() * (self.time2 - self.time1),
        )
    }
}
