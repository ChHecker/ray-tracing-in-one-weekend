use std::path::Path;
use std::sync::Arc;

use rand::Rng;
use ray_tracing_in_one_weekend::materials::*;
use ray_tracing_in_one_weekend::shapes::*;
use ray_tracing_in_one_weekend::*;

fn random_world(world: &mut HittableList) {
    let mut rng = rand::thread_rng();

    let ground_material = Arc::new(Lambertian::new(color![0.5, 0.5, 0.5]));
    let ground_sphere = Arc::new(Sphere::new(point![0., -1000., 0.], 1000., ground_material));
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
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.9 {
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = 0.5 * rng.gen::<f32>();
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Arc::new(Sphere::new(point![0., 1., 0.], 1., material1));
    world.push(sphere1);

    let material2 = Arc::new(Lambertian::new(color![0.4, 0.2, 0.1]));
    let sphere2 = Arc::new(Sphere::new(point![-4., 1., 0.], 1., material2));
    world.push(sphere2);

    let material3 = Arc::new(Metal::new(color![0.7, 0.6, 0.5], 0.));
    let sphere3 = Arc::new(Sphere::new(point![3., 1., 0.], 1., material3).with_time(
        point![5., 1., 0.],
        0.,
        1.,
    ));
    world.push(sphere3);
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: u16 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 50;
    let max_depth = 20;

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
    );
    random_world(&mut raytracer.world);

    raytracer
        .render()
        .save(&Path::new("images/book2-chapter3.png"))
        .unwrap();
}
