use std::path::Path;
use std::sync::Arc;

use indicatif::ProgressBar;
use rand::Rng;
use ray_tracing_in_one_weekend::*;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub fn ray_color(world: &HittableList, ray: &Ray, depth: usize) -> Color {
    if depth == 0 {
        return color![0., 0., 0.];
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        let target = hit.point() + hit.normal() + Point3::random_in_hemisphere(&hit.normal()); // + Point3::random_unit_vector();
        return 0.5
            * ray_color(
                world,
                &Ray::new(hit.point(), target - hit.point()),
                depth - 1,
            );
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color![1., 1., 1.] + t * color![0.5, 0.7, 1.0]
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    world.push(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.push(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Progressbar
    let bar = ProgressBar::new((image_height * image_width).try_into().unwrap());

    let mut ppm = vec![color![0., 0., 0.]; image_height * image_width];
    ppm.par_iter_mut().enumerate().for_each(|(index, color)| {
        let mut rng = rand::thread_rng();
        let i = index % image_width;
        let j = image_height - index / image_width - 1;

        let mut pixel_color = color![0., 0., 0.];

        for _ in 0..samples_per_pixel {
            let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
            let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
            pixel_color += ray_color(&world, &camera.get_ray(u, v), max_depth);
        }
        pixel_color = color!(
            (pixel_color.x() / samples_per_pixel as f32).sqrt(),
            (pixel_color.y() / samples_per_pixel as f32).sqrt(),
            (pixel_color.z() / samples_per_pixel as f32).sqrt(),
        );

        bar.inc(1);

        *color = pixel_color;
    });

    write_ppm(
        &Path::new("images/ppm8_hemisphere.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
