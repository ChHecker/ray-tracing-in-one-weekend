use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_in_one_weekend::materials::Lambertian;
use ray_tracing_in_one_weekend::shapes::Sphere;
use ray_tracing_in_one_weekend::*;

fn criterion_benchmark(c: &mut Criterion) {
    let sphere = Sphere::new(
        point![0., 1., -1.],
        1.,
        Lambertian::solid_color(color![1., 1., 1.]),
    );

    c.bench_function("Sphere", |b| {
        b.iter_batched(
            || {
                let mut raytracer = Raytracer::new(Camera::default(), 160, 90, 10, 10);
                raytracer.world.push(sphere.clone());
                raytracer
            },
            |rt| rt.render(),
            SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
