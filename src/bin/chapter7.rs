use std::{path::Path, sync::Arc};

use rand::Rng;
use raytracing_in_one_weekend::*;

pub fn ray_color(world: &HittableList, ray: &Ray) -> Color3 {
    if let Some(hit) = world.hit(ray, 0., f64::INFINITY) {
        let normal = hit.normal();
        return 0.5 * Color3::new(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1., 1., 1.) + t * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    world.push(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.push(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut rng = rand::thread_rng();

    let mut ppm = Vec::<Color3>::new();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color3::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                pixel_color += ray_color(&world, &camera.get_ray(u, v));
            }
            pixel_color /= samples_per_pixel as f64;
            ppm.push(pixel_color);
        }
    }
    write_ppm(
        &Path::new("images/ppm7.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
