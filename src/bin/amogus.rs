use std::path::Path;

use ray_tracing_in_one_weekend::materials::*;
use ray_tracing_in_one_weekend::shapes::*;
use ray_tracing_in_one_weekend::textures::*;
use ray_tracing_in_one_weekend::*;

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

    let red_texture = SolidColor::new(color![1., 0., 0.]);
    let red_lambertian = Lambertian::new(&red_texture);

    let green_texture = SolidColor::new(color![0., 1., 0.]);
    let green_lambertian = Lambertian::new(&green_texture);

    let ground = Sphere::new(point![0., -1000., -1.], 1000., &green_lambertian);
    raytracer.world.push(Box::new(ground));

    let left_leg = Cylinder::new(point![-1., 0.5, -1.], 0.5, 1., &red_lambertian);
    raytracer.world.push(Box::new(left_leg));

    let right_leg = Cylinder::new(point![1., 0.5, -1.], 0.5, 1., &red_lambertian);
    raytracer.world.push(Box::new(right_leg));

    let body = Cylinder::new(point![0., 2.5, -1.], 1.5, 3., &red_lambertian);
    raytracer.world.push(Box::new(body));

    let head = Sphere::new(point![0., 4., -1.], 1.5, &red_lambertian);
    raytracer.world.push(Box::new(head));

    let blue_texture = SolidColor::new(color![0.3, 0.3, 1.]);
    let blue_metal = Metal::new(&blue_texture, 5.);
    let visor = Sphere::new(point![0., 4., 0.], 0.8, &blue_metal);
    raytracer.world.push(Box::new(visor));

    let dark_red_texture = SolidColor::new(color![0.8, 0., 0.]);
    let dark_red_lambertian = Lambertian::new(&dark_red_texture);
    let backpack = Cylinder::new(point![0., 3., -2.75], 0.5, 1.8, &dark_red_lambertian);
    raytracer.world.push(Box::new(backpack));

    // let glass_sphere = Sphere::new(point![0., 3., -1.], 4., Arc::new(Dielectric::new(1.5)));
    // world.push(Arc::new(glass_sphere));

    // let inner_glass_sphere = Sphere::new(point![0., 3., -1.], -3.9, Arc::new(Dielectric::new(1.5)));
    // world.push(Arc::new(inner_glass_sphere));

    raytracer
        .render()
        .save(&Path::new("images/amogus.png"))
        .unwrap();
}
