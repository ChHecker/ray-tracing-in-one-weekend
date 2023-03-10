use std::path::Path;
use std::sync::Arc;

use rand::Rng;
use ray_tracing_in_one_weekend::*;

fn demo_world(world: &mut HittableList) {
    let ground = Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(color![0., 1., 0.])),
    );
    world.push(Arc::new(ground));

    let diffusive_sphere = Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(color![1., 0., 0.])),
    );
    world.push(Arc::new(diffusive_sphere));

    let metal_sphere = Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Arc::new(Metal::new(color![0.2, 0.2, 0.2], 1.)),
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

    let ground_material = Arc::new(Lambertian::new(color![0.5, 0.5, 0.5]));
    let ground_sphere = Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));
    world.push(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f32 = rng.gen();
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            let sphere_material: Arc<dyn Material + Send + Sync>;

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_material < 0.9 {
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = 0.5 * rng.gen::<f32>();
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
    world.push(sphere1);

    let material2 = Arc::new(Lambertian::new(color![0.4, 0.2, 0.1]));
    let sphere2 = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
    world.push(sphere2);

    let material3 = Arc::new(Metal::new(color![0.7, 0.6, 0.5], 0.));
    let sphere3 = Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material3));
    world.push(sphere3);
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
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
        std::f32::consts::FRAC_PI_6,
        aspect_ratio,
        0.1,
        10.,
    );

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
        .write_ppm(&Path::new("images/ppm10.ppm"))
        .unwrap();
}
