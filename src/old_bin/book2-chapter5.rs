use std::path::Path;

use rand::Rng;
use ray_tracing_in_one_weekend::color::{BLACK, WHITE};
use ray_tracing_in_one_weekend::materials::*;
use ray_tracing_in_one_weekend::shapes::*;
use ray_tracing_in_one_weekend::textures::*;
use ray_tracing_in_one_weekend::*;

#[allow(dead_code)]
fn random_world(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    let mut rng = rand::thread_rng();

    // Camera
    let lookfrom = point![13., 2., 3.];
    let lookat = point![0., 0., 0.];
    let vup = point![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_6,
        aspect_ratio,
        0.1,
        10.,
    )
    .with_time(0., 1.);

    let mut raytracer = Raytracer::new(
        camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();
    let world = &mut raytracer.world;

    let ground_material = Lambertian::new(CheckerTexture::solid_colors(WHITE, BLACK));
    let ground_sphere = Sphere::new(point![0., -1000., 0.], 1000., ground_material);
    world.push(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f32 = rng.gen();
            let center = point!(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - point![4., 0.2, 0.]).norm() > 0.9 {
                if choose_material < 0.8 {
                    let sphere_material =
                        Lambertian::solid_color(Color::random() * Color::random());
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_material < 0.9 {
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = 0.5 * rng.gen::<f32>();
                    let sphere_material = Metal::solid_color(albedo, fuzz);
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(point![0., 1., 0.], 1., material1);
    world.push(sphere1);

    let material2 = Lambertian::new(SolidColor::new(color![0.4, 0.2, 0.1]));
    let sphere2 = Sphere::new(point![-4., 1., 0.], 1., material2);
    world.push(sphere2);

    let material3 = Metal::solid_color(color![0.7, 0.6, 0.5], 0.);
    let sphere3 =
        Sphere::new(point![3., 1., 0.], 1., material3).with_time(point![5., 1., 0.], 0., 1.);
    world.push(sphere3);

    raytracer
}

#[allow(dead_code)]
fn checkerboard_world(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = point![13., 2., 3.];
    let lookat = point![0., 0., 0.];
    let vup = point![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_6,
        aspect_ratio,
        0.1,
        10.,
    )
    .with_time(0., 1.);

    let mut raytracer = Raytracer::new(
        camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();
    let world = &mut raytracer.world;

    let checker = CheckerTexture::solid_colors(WHITE, BLACK);
    world.push(Sphere::new(
        point![0., -10., 0.],
        10.,
        Lambertian::new(checker.clone()),
    ));
    world.push(Sphere::new(
        point![0., 10., 0.],
        10.,
        Lambertian::new(checker),
    ));

    raytracer
}

#[allow(dead_code)]
fn perlin(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = point![13., 2., 3.];
    let lookat = point![0., 0., 0.];
    let vup = point![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_8,
        aspect_ratio,
        0.1,
        10.,
    )
    .with_time(0., 1.);

    let mut raytracer = Raytracer::new(
        camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;
    let perlin_lambertian = Lambertian::new(PerlinNoiseTexture::new(4.));
    world.push(Sphere::new(
        point![0., -1000., 0.],
        1000.,
        perlin_lambertian.clone(),
    ));
    world.push(Sphere::new(point![0., 2., 0.], 2., perlin_lambertian));

    raytracer
}

#[allow(dead_code)]
enum Scene {
    Random,
    Checkerboard,
    Perlin,
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: u16 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 50;
    let max_depth = 20;

    let path: &Path;

    let scene = Scene::Perlin;
    let raytracer = match scene {
        Scene::Random => {
            path = Path::new("images/book2-chapter5-random.png");
            random_world(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Checkerboard => {
            path = Path::new("images/book2-chapter5-checkerboard.png");
            checkerboard_world(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Perlin => {
            path = Path::new("images/book2-chapter5-perlin.png");
            perlin(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
    };

    raytracer.render().save(path).unwrap();
}
