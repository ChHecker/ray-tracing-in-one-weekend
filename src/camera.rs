use crate::*;
use rand::Rng;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
    u: Point,
    v: Point,
    w: Point,
    lens_radius: f32,
    time: Option<(f32, f32)>,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Point,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
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

    pub fn with_time(mut self, time_start: f32, time_end: f32) -> Self {
        self.time = Some((time_start, time_end));
        self
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let mut rng = rand::thread_rng();

        let random_disk = self.lens_radius * Point::random_in_unit_disk();
        let offset = self.u * random_disk.x() + self.v * random_disk.y();

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
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            point!(0., 0., 0.),
            point!(0., 0., -1.),
            point!(0., 1., 0.),
            std::f32::consts::FRAC_PI_6,
            16. / 9.,
            0.,
            1.,
        )
    }
}
