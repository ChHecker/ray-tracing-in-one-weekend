use std::path::Path;

use raytracing_in_one_weekend::{ppm::write_ppm_vec, *};

pub fn ray_color(ray: &Ray) -> Color3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1., 1., 1.) + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let width: usize = 400;
    let height = (width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Point3::new(viewport_width, 0., 0.);
    let vertical = Point3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Point3::new(0., 0., focal_length);

    let mut ppm = Vec::<Vec<Color3>>::new();
    for j in (0..height).rev() {
        let mut row = Vec::<Color3>::new();
        for i in 0..width {
            let u = i as f64 / 255.;
            let v = j as f64 / 255.;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            row.push(ray_color(&ray));
        }
        ppm.push(row);
    }
    write_ppm_vec(&Path::new("images/ppm4.ppm"), (width, height), ppm).unwrap();
}
