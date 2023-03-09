use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_in_one_weekend::materials::Lambertian;
use ray_tracing_in_one_weekend::shapes::Sphere;
use ray_tracing_in_one_weekend::textures::SolidColor;
use ray_tracing_in_one_weekend::*;

fn criterion_benchmark(c: &mut Criterion) {
    let texture = SolidColor::new(color![1., 1., 1.]);
    let material = Lambertian::new(&texture);

    c.bench_function("Sphere", |b| {
        b.iter_batched(
            || {
                let mut raytracer = Raytracer::new(Camera::default(), 160, 90, 10, 10);
                let sphere = Sphere::new(point![0., 1., -1.], 1., &material);
                raytracer.world.push(Box::new(sphere));
                raytracer
            },
            |rt| rt.render(),
            SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
