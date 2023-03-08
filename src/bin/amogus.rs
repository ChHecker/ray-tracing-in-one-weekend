use ray_tracing_in_one_weekend::{materials::*, shapes::*, *};
use std::{path::Path, sync::Arc};

fn amogus(world: &mut HittableList) {
    let ground = Sphere::new(
        point![0., -1000., -1.],
        1000.,
        Arc::new(Lambertian::new(color![0., 1., 0.])),
    );
    world.push(Arc::new(ground));

    let left_leg = Cylinder::new(
        point![-1., 0.5, -1.],
        0.5,
        1.,
        Arc::new(Lambertian::new(color![1., 0., 0.])),
    );
    world.push(Arc::new(left_leg));

    let right_leg = Cylinder::new(
        point![1., 0.5, -1.],
        0.5,
        1.,
        Arc::new(Lambertian::new(color![1., 0., 0.])),
    );
    world.push(Arc::new(right_leg));

    let body = Cylinder::new(
        point![0., 2.5, -1.],
        1.5,
        3.,
        Arc::new(Lambertian::new(color![1., 0., 0.])),
    );
    world.push(Arc::new(body));

    let head = Sphere::new(
        point![0., 4., -1.],
        1.5,
        Arc::new(Lambertian::new(color![1., 0., 0.])),
    );
    world.push(Arc::new(head));

    let visor = Sphere::new(
        point![0., 4., 0.],
        0.8,
        Arc::new(Metal::new(color![0.3, 0.3, 1.], 5.)),
    );
    world.push(Arc::new(visor));

    let backpack = Cylinder::new(
        point![0., 3., -2.75],
        0.5,
        1.8,
        Arc::new(Lambertian::new(color![0.8, 0., 0.])),
    );
    world.push(Arc::new(backpack));

    let glass_sphere = Sphere::new(point![0., 3., -1.], 4., Arc::new(Dielectric::new(1.5)));
    world.push(Arc::new(glass_sphere));

    let inner_glass_sphere = Sphere::new(point![0., 3., -1.], -3.9, Arc::new(Dielectric::new(1.5)));
    world.push(Arc::new(inner_glass_sphere));
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width: u16 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 100;
    let max_depth = 10;

    // Camera
    let lookfrom = point![7., 0., 3.];
    let lookat = point![0., 3., -1.];
    let vup = point![0., 1., 0.];
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

    amogus(&mut raytracer.world);

    raytracer
        .render()
        .save(&Path::new("images/amogus.png"))
        .unwrap();
}
