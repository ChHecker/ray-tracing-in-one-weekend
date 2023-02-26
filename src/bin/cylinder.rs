use raytracing_in_one_weekend::{shapes::ZCylinder, *};
use std::{path::Path, sync::Arc};

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: usize = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 200;
    let max_depth = 100;

    // Camera
    let lookfrom = Point3::new(0., 0., 2.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Point3::new(0., 1., 0.);
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f64::consts::FRAC_PI_4,
        aspect_ratio,
        0.,
        1.,
        0.,
        0.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );

    let cylinder = ZCylinder::new(
        Point3::new(-1., 0., -1.),
        0.5,
        0.5,
        Arc::new(Lambertian::new(Color::new(1., 0., 0.))),
    );
    raytracer.world.push(Arc::new(cylinder));

    let sphere = Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(Color::new(0., 0., 1.))),
    );
    raytracer.world.push(Arc::new(sphere));

    let ground = Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(Color::new(0., 1., 0.))),
    );
    raytracer.world.push(Arc::new(ground));

    raytracer
        .render()
        .write_ppm(&Path::new("images/cylinder.ppm"))
        .unwrap();
}
