use rand::Rng;
use raytracing_in_one_weekend::{materials::*, shapes::*, *};
use std::{path::Path, sync::Arc};

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

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
    world.push(sphere1);

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere2 = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
    world.push(sphere2);

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    let sphere3 = Arc::new(Sphere::new_with_time(
        (Point3::new(3., 1., 0.), Point3::new(5., 1., 0.)),
        (0., 1.),
        1.,
        material3,
    ));
    world.push(sphere3);
}

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Point3::new(0., 1., 0.);
    let camera = Camera::new_with_time(
        lookfrom,
        lookat,
        vup,
        std::f64::consts::FRAC_PI_6,
        aspect_ratio,
        0.1,
        10.,
        (0., 1.),
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
        .save(&Path::new("images/book2-chapter2.png"))
        .unwrap();
}
