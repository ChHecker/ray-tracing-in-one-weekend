use std::path::Path;

use ray_tracing_in_one_weekend::*;

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color!(1., 1., 1.) + t * color!(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width: usize = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Point3::new(viewport_width, 0., 0.);
    let vertical = Point3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Point3::new(0., 0., focal_length);

    let mut ppm = Vec::<Color>::new();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            ppm.push(ray_color(&ray));
        }
    }
    write_ppm(
        &Path::new("images/ppm4.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
