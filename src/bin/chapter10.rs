use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use raytracing_in_one_weekend::*;
use std::{path::Path, sync::Arc};

fn ray_color(world: &HittableList, ray: Ray, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
            return attenuation * ray_color(&world, scattered, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn demo_world(world: &mut HittableList) {
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
}

fn random_world(world: &mut HittableList) {
    let mut rng = rand::thread_rng();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));
    world.push(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            let sphere_material: Arc<dyn Material + Send + Sync>;

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_material < 0.9 {
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = 0.5 * rng.gen::<f64>();
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // auto material1 = make_shared<dielectric>(1.5);
    // world.add(make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    // auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
    // world.add(make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    // auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    // world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));
    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
    world.push(sphere1);

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere2 = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
    world.push(sphere2);

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    let sphere3 = Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material3));
    world.push(sphere3);
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 500;
    let max_depth = 100;

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Point3::new(0., 1., 0.);
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f64::consts::FRAC_PI_6,
        aspect_ratio,
        0.1,
        10.,
    );

    // World
    let mut world = HittableList::new();
    random_world(&mut world);

    // Progressbar
    let bar = ProgressBar::new((image_height * image_width).try_into().unwrap());
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut ppm = vec![Color::new(0., 0., 0.); image_height * image_width];
    ppm.par_iter_mut().enumerate().for_each(|(index, color)| {
        let mut rng = rand::thread_rng();
        let i = index % image_width;
        let j = image_height - index / image_width - 1;

        let mut pixel_color = Color::new(0., 0., 0.);

        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
            let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
            pixel_color += ray_color(&world, camera.get_ray(u, v), max_depth);
        }
        pixel_color = Color::new(
            (pixel_color.x() / samples_per_pixel as f64).sqrt(),
            (pixel_color.y() / samples_per_pixel as f64).sqrt(),
            (pixel_color.z() / samples_per_pixel as f64).sqrt(),
        );

        bar.inc(1);

        *color = pixel_color;
    });

    write_ppm(
        &Path::new("images/ppm10.ppm"),
        (image_width, image_height),
        &ppm,
    )
    .unwrap();
}
