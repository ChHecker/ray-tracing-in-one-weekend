use ray_tracing_in_one_weekend::{
    materials::Lambertian,
    shapes::{Cylinder, Sphere},
    *,
};
use std::{path::Path, sync::Arc};

fn main() {
    // Image
    let aspect_ratio = 16. / 10.;
    let image_width: u16 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 200;
    let max_depth = 100;

    // Camera
    let lookfrom = point!(0., 0., 2.);
    let lookat = point!(0., 0., 0.);
    let vup = point!(0., 1., 0.);
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_4,
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

    let cylinder = Cylinder::new(
        point!(-1., 0., -1.),
        0.5,
        0.5,
        Arc::new(Lambertian::new(color!(1., 0., 0.))),
    );
    raytracer.world.push(Arc::new(cylinder));

    let sphere = Sphere::new(
        point!(1., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(color!(0., 0., 1.))),
    );
    raytracer.world.push(Arc::new(sphere));

    let ground = Sphere::new(
        point!(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(color!(0., 1., 0.))),
    );
    raytracer.world.push(Arc::new(ground));

    raytracer
        .render()
        .save(&Path::new("images/cylinder.png"))
        .unwrap();
}
