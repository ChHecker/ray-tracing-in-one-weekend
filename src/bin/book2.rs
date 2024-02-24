use std::path::Path;

use nalgebra::Rotation3;
#[allow(unused_imports)]
use rand::Rng;
use ray_tracing_in_one_weekend::color::{BLACK, GREEN, RED, WHITE};
use ray_tracing_in_one_weekend::materials::*;
use ray_tracing_in_one_weekend::shapes::*;
use ray_tracing_in_one_weekend::textures::*;
use ray_tracing_in_one_weekend::vec3::random_vector_in_range;
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
    let lookfrom = vector![13., 2., 3.];
    let lookat = vector![0., 0., 0.];
    let vup = vector![0., 1., 0.];
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
        color![0.7, 0.808, 0.922],
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();
    let world = &mut raytracer.world;

    let ground_material = Lambertian::new(CheckerTexture::solid_colors(WHITE, BLACK));
    let ground_sphere = Sphere::new(vector![0., -1000., 0.], 1000., ground_material);
    world.push(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f32 = rng.gen();
            let center = vector![
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>()
            ];

            if (center - vector![4., 0.2, 0.]).norm() > 0.9 {
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
    let sphere1 = Sphere::new(vector![0., 1., 0.], 1., material1);
    world.push(sphere1);

    let material2 = Lambertian::solid_color(color![0.4, 0.2, 0.1]);
    let sphere2 = Sphere::new(vector![-4., 1., 0.], 1., material2);
    world.push(sphere2);

    let material3 = Metal::solid_color(color![0.7, 0.6, 0.5], 0.);
    let sphere3 =
        Sphere::new(vector![3., 1., 0.], 1., material3).moving(vector![5., 1., 0.], 0., 1.);
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
    let lookfrom = vector![13., 2., 3.];
    let lookat = vector![0., 0., 0.];
    let vup = vector![0., 1., 0.];
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
        color![0.7, 0.808, 0.922],
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();
    let world = &mut raytracer.world;

    let checker = CheckerTexture::solid_colors(WHITE, BLACK);
    world.push(Sphere::new(
        vector![0., -10., 0.],
        10.,
        Lambertian::new(checker.clone()),
    ));
    world.push(Sphere::new(
        vector![0., 10., 0.],
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
    let lookfrom = vector![13., 2., 3.];
    let lookat = vector![0., 0., 0.];
    let vup = vector![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_8,
        aspect_ratio,
        0.1,
        10.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        color![0.7, 0.808, 0.922],
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;
    let perlin_lambertian = Lambertian::new(PerlinNoiseTexture::new(4.));
    world.push(Sphere::new(
        vector![0., -1000., 0.],
        1000.,
        perlin_lambertian.clone(),
    ));
    world.push(Sphere::new(vector![0., 2., 0.], 2., perlin_lambertian));

    raytracer
}

#[allow(dead_code)]
fn image(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = vector![13., 2., 3.];
    let lookat = vector![0., 0., 0.];
    let vup = vector![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_8,
        aspect_ratio,
        0.1,
        10.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        color![0.7, 0.808, 0.922],
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;
    let image_texture = ImageTexture::open(Path::new("link.png")).unwrap();
    let image_material = Metal::new(image_texture, 1.);
    let sphere = Sphere::new(vector![0., 0., 0.], 2., image_material);
    world.push(sphere);

    raytracer
}

#[allow(dead_code)]
fn light(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = vector![26., 3., 9.];
    let lookat = vector![0., 2., 0.];
    let vup = vector![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_6,
        aspect_ratio,
        0.,
        1.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        BLACK,
        // color![0.1, 0.1, 0.15],
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;

    // let noise = Lambertian::new(PerlinNoiseTexture::new(4.));
    let red = Lambertian::solid_color(RED);
    let green = Lambertian::solid_color(GREEN);
    let light = DiffuseLight::solid_color(4. * WHITE);

    // let sphere1 = Sphere::new(vector![0., -1000., 0.], 1000., noise.clone());
    // world.push(sphere1);
    // let sphere2 = Sphere::new(vector![-2., 5., 0.], 2., noise.clone());
    // world.push(sphere2);
    let sphere1 = Sphere::new(vector![-2., 5., 0.], 2., green.clone());
    world.push(sphere1);
    let sphere2 = Sphere::new(vector![-2., -2., 0.], 2., green.clone());
    world.push(sphere2);

    let cylinder = Cylinder::new(vector![-2., 1.5, 0.], 0.3, 3., light);
    world.push(cylinder);
    // let rectangle1 = Rectangle::xy(vector![-2., 1.5, 0.], 3., 3., light);
    // world.push(rectangle1);
    let rectangle2 = Rectangle::xy(vector![-2., 1.5, -4.], 100., 100., red);
    world.push(rectangle2);
    let rectangle3 = Rectangle::xy(vector![-2., 1.5, 5.], 3., 3., green)
        .with_rotation(Rotation3::new(Vector3::y()));
    world.push(rectangle3);

    raytracer
}

#[allow(dead_code)]
fn cornell(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = vector![0., 0., 250.];
    let lookat = vector![0., 0., 0.];
    let vup = vector![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f32::consts::FRAC_PI_2,
        aspect_ratio,
        0.,
        1.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        BLACK,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;

    let red = Lambertian::solid_color(color![0.65, 0.05, 0.05]);
    let white = Lambertian::solid_color(color![0.73, 0.73, 0.73]);
    let green = Lambertian::solid_color(color![0.15, 0.45, 0.15]);
    let light = DiffuseLight::solid_color(5. * WHITE);

    let floor = Rectangle::xz(vector![0., -200., 0.], 400., 400., white.clone());
    let roof = Rectangle::xz(vector![0., 200., 0.], 400., 400., white.clone());
    let back_wall = Rectangle::xy(vector![0., 0., -200.], 400., 400., white.clone());
    let left_wall = Rectangle::yz(vector![-200., 0., 0.], 400., 400., green);
    let right_wall = Rectangle::yz(vector![200., 0., 0.], 400., 400., red);
    let light_rect = Rectangle::xz(vector![0., 200., 0.], 200., 200., light);

    let box1 = Cuboid::new(vector![30., -75., -50.], 100., 150., 100., white.clone())
        .with_rotation(Rotation3::new((15f32).to_radians() * Vector3::y()));
    let dust_box1 = ConstantMedium::solid_color(box1, WHITE, 0.01);
    let box2 = Cuboid::new(vector![-20., -50., -100.], 120., 300., 120., white.clone())
        .with_rotation(Rotation3::new((-18f32).to_radians() * Vector3::y()));
    let dust_box2 = ConstantMedium::solid_color(box2, BLACK, 0.01);

    world.push(floor);
    world.push(roof);
    world.push(back_wall);
    world.push(left_wall);
    world.push(right_wall);
    world.push(light_rect);
    world.push(dust_box1);
    world.push(dust_box2);

    raytracer
}

#[allow(dead_code)]
fn final_scene(
    aspect_ratio: f32,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Raytracer {
    // Camera
    let lookfrom = vector![478., 278., -600.];
    let lookat = vector![278., 278., 0.];
    let vup = vector![0., 1., 0.];
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        (40f32).to_radians(),
        aspect_ratio,
        0.,
        1.,
    );

    let mut raytracer = Raytracer::new(
        camera,
        BLACK,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
    .with_progressbar();

    let world = &mut raytracer.world;

    let mut rng = rand::thread_rng();

    let ground = Lambertian::solid_color(color![0.48, 0.83, 0.53]);

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f32 * w;
            let y0 = 0.;
            let z0 = -1000. + j as f32 * w;
            let x1 = x0 + w / 2.;
            let y1 = rng.gen_range(1.0..101.) / 2.;
            let z1 = z0 + w / 2.;

            world.push(Cuboid::new(
                vector![x1, y1, z1],
                2. * (x1 - x0).abs(),
                2. * (y1 - y0).abs(),
                2. * (z1 - z0).abs(),
                ground.clone(),
            ))
        }
    }

    let light = DiffuseLight::solid_color(7. * WHITE);
    world.push(Rectangle::xz(
        vector![273., 554., 279.5],
        150.,
        132.5,
        light,
    ));

    let center1 = vector![400., 400., 200.];
    let center2 = center1 + vector![30., 0., 0.];
    let moving_sphere_material = Lambertian::solid_color(color![0.7, 0.4, 0.1]);

    world.push(Sphere::new(center1, 50., moving_sphere_material).moving(center2, 0., 1.));
    world.push(Sphere::new(
        vector![260., 150., 45.],
        50.,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        vector![0., 150., 145.],
        50.,
        Metal::solid_color(color![0.8, 0.8, 0.9], 1.),
    ));

    let boundary = Sphere::new(vector![360., 150., 145.], 70., Dielectric::new(1.5));
    world.push(boundary.clone());
    world.push(ConstantMedium::solid_color(
        boundary,
        color![0.2, 0.4, 0.9],
        0.2,
    ));
    let boundary = Sphere::new(vector![0., 0., 0.], 5000., Dielectric::new(1.5));
    world.push(ConstantMedium::solid_color(boundary, BLACK, 0.0001));

    let link = Lambertian::new(ImageTexture::open("link.png").unwrap());
    world.push(Sphere::new(vector![400., 200., 400.], 100., link));
    let pertext = PerlinNoiseTexture::new(0.1);
    world.push(Sphere::new(
        vector![220., 280., 300.],
        80.,
        Lambertian::new(pertext),
    ));

    let mut boxes2 = HittableList::new(vector![-100., 270., 395.]);
    let white = Lambertian::solid_color(color![0.73, 0.73, 0.73]);
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Sphere::new(
            random_vector_in_range(0., 165.),
            10.,
            white.clone(),
        ));
    }
    boxes2 = boxes2.with_rotation(Rotation3::new((15f32).to_radians() * Vector3::y()));

    world.push(boxes2);

    raytracer
}

#[allow(dead_code)]
enum Scene {
    Random,
    Checkerboard,
    Perlin,
    Image,
    Light,
    Cornell,
    Final,
}

fn main() {
    // Image
    let aspect_ratio = 1.;
    let image_width: u16 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u16;
    let samples_per_pixel: u16 = 100;
    let max_depth = 20;

    let path: &Path;

    let scene = Scene::Final;
    let raytracer = match scene {
        Scene::Random => {
            path = Path::new("images/book2-chapter4-random.png");
            random_world(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Checkerboard => {
            path = Path::new("images/book2-chapter4-checkerboard.png");
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
        Scene::Image => {
            path = Path::new("images/book2-chapter6-image.png");
            image(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Light => {
            path = Path::new("images/book2-chapter7-light.png");
            light(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Cornell => {
            path = Path::new("images/book2-chapter7-cornell.png");
            cornell(
                aspect_ratio,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }
        Scene::Final => {
            path = Path::new("images/book2-chapter10-final.png");
            final_scene(
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
