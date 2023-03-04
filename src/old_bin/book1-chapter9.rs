use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use raytracing_in_one_weekend::*;
use std::{path::Path, sync::Arc};

pub fn ray_color(world: &HittableList, ray: &Ray, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
            return attenuation * ray_color(&world, &scattered, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 100;

    // Camera
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();

    let ground = Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(Color::new(0., 1., 0.))),
    );
    world.push(Arc::new(ground));

    let diffusive_sphere = Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    world.push(Arc::new(diffusive_sphere));

    let metal_sphere = Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Arc::new(Metal::new(Color::new(0.2, 0.2, 0.2), 1.)),
    );
    world.push(Arc::new(metal_sphere));

    let glass_sphere = Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    );
    world.push(Arc::new(glass_sphere));

    let inner_glass_sphere = Sphere::new(
        Point3::new(1., 0., -1.),
        -0.4,
        Arc::new(Dielectric::new(1.5)),
    );
    world.push(Arc::new(inner_glass_sphere));

    // Progressbar
    let bar = ProgressBar::new((image_height * image_width).try_into().unwrap());

    let mut ppm = vec![Color::new(0., 0., 0.); image_height * image_width];
    ppm.par_iter_mut().enumerate().for_each(|(index, color)| {
        let mut rng = rand::thread_rng();
        let i = index % image_width;
        let j = image_height - index / image_width - 1;

        let mut pixel_color = Color::new(0., 0., 0.);

        for _ in 0..samples_per_pixel {
            let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
            let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
            pixel_color += ray_color(&world, &camera.get_ray(u, v), max_depth);
        }
        pixel_color = Color::new(
            (pixel_color.x() / samples_per_pixel as f32).sqrt(),
            (pixel_color.y() / samples_per_pixel as f32).sqrt(),
            (pixel_color.z() / samples_per_pixel as f32).sqrt(),
        );

        bar.inc(1);

        *color = pixel_color;
    });

    write_ppm(
        &Path::new("images/ppm9.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
