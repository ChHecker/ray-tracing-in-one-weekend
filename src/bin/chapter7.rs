use std::{path::Path, rc::Rc};

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

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    world.push(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.push(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut ppm = Vec::<Color3>::new();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            ppm.push(ray_color(&world, &camera.get_ray(u, v)));
        }
    }
    write_ppm(
        &Path::new("images/ppm6-4.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
