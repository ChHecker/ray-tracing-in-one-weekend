use raytracing_in_one_weekend::{materials::*, shapes::*, *};
use std::{path::Path, sync::Arc};

fn amogus(world: &mut HittableList) {
    let ground = Sphere::new(
        Point3::new(0., -1000., -1.),
        1000.,
        Arc::new(Lambertian::new(Color::new(0., 1., 0.))),
    );
    world.push(Arc::new(ground));

    let left_leg = Cylinder::new(
        Point3::new(-1., 0.5, -1.),
        0.5,
        1.,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    world.push(Arc::new(left_leg));

    let right_leg = Cylinder::new(
        Point3::new(1., 0.5, -1.),
        0.5,
        1.,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    world.push(Arc::new(right_leg));

    let body = Cylinder::new(
        Point3::new(0., 2.5, -1.),
        1.5,
        3.,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    world.push(Arc::new(body));

    let head = Sphere::new(
        Point3::new(0., 4., -1.),
        1.5,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    world.push(Arc::new(head));

    let visor = Sphere::new(
        Point3::new(0., 4., 0.),
        0.8,
        Arc::new(Metal::new(Color::new(0.3, 0.3, 1.), 5.)),
    );
    world.push(Arc::new(visor));

    let backpack = Cylinder::new(
        Point3::new(0., 3., -2.75),
        0.5,
        1.8,
        Arc::new(Lambertian::new(Color::new(0.8, 0., 0.))),
    );
    world.push(Arc::new(backpack));

    let glass_sphere = Sphere::new(Point3::new(0., 3., -1.), 4., Arc::new(Dielectric::new(1.5)));
    world.push(Arc::new(glass_sphere));

    let inner_glass_sphere = Sphere::new(
        Point3::new(0., 3., -1.),
        -3.9,
        Arc::new(Dielectric::new(1.5)),
    );
    world.push(Arc::new(inner_glass_sphere));
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width: usize = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 10;

    // Camera
    let lookfrom = Point3::new(7., 0., 3.);
    let lookat = Point3::new(0., 3., -1.);
    let vup = Point3::new(0., 1., 0.);
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f64::consts::FRAC_PI_4,
        aspect_ratio,
        0.,
        1.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );

    amogus(&mut raytracer.world);

    raytracer
        .render()
        .save(&Path::new("images/amogus.png"))
        .unwrap();
}
