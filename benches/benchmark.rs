use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use nalgebra::Rotation3;
use ray_tracing_in_one_weekend::color::BLACK;
use ray_tracing_in_one_weekend::materials::Lambertian;
use ray_tracing_in_one_weekend::shapes::{Cuboid, Movable, Sphere};
use ray_tracing_in_one_weekend::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut raytracer = Raytracer::new(Camera::default(), BLACK, 160, 90, 10, 10);

    let sphere = Sphere::new(
        vector![0., 1., -1.],
        1.,
        Lambertian::solid_color(color![1., 1., 1.]),
    )
    .with_rotation(Rotation3::new(Vector3::y()));
    raytracer.world.push(sphere);

    let cuboid = Cuboid::new(
        vector![2., 0., 0.],
        1.,
        1.,
        1.,
        Lambertian::solid_color(color![1., 1., 1.]),
    )
    .with_rotation(Rotation3::new(Vector3::z()));
    raytracer.world.push(cuboid);

    c.bench_function("World", |b| {
        b.iter_batched(|| raytracer.clone(), |rt| rt.render(), SmallInput);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
