use crate::*;
use rand::Rng;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    u: Point3,
    v: Point3,
    w: Point3,
    lens_radius: f64,
    time: Option<(f64, f64)>,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Point3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
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
            time: None,
        }
    }

    pub fn with_time(mut self, time_start: f64, time_end: f64) -> Self {
        self.time = Some((time_start, time_end));
        self
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let random_disk = self.lens_radius * Point3::random_in_unit_disk();
        let offset = self.u * random_disk.x() + self.v * random_disk.y();

        let ray = Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        );
        if let Some((time1, time2)) = self.time {
            ray.with_time(time1 + rng.gen::<f64>() * (time2 - time1))
        } else {
            ray
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::new(0., 0., 0.),
            Point3::new(0., 0., -1.),
            Point3::new(0., 1., 0.),
            std::f64::consts::FRAC_PI_6,
            16. / 9.,
            0.,
            1.,
        )
    }
}
