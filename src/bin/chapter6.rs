use std::path::Path;

use raytracing_in_one_weekend::*;

pub fn ray_color(ray: &Ray) -> Color3 {
    if let Some(t) = sphere::hits_sphere(ray, &Point3::new(0., 0., -1.), 0.5) {
        let normal = (ray.at(t) - Point3::new(0., 0., -1.)).unit_vector();
        return 0.5 * Color3::new(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1., 1., 1.) + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Point3::new(viewport_width, 0., 0.);
    let vertical = Point3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Point3::new(0., 0., focal_length);

    let mut ppm = Vec::<Color3>::new();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            ppm.push(ray_color(&ray));
        }
    }
    write_ppm(
        &Path::new("images/ppm6.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
