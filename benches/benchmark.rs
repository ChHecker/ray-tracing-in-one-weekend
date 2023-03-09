use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_in_one_weekend::*;

fn criterion_benchmark(c: &mut Criterion) {
    let camera = Camera::default();
    let raytracer = Raytracer::new(camera, 160, 90, 10, 10);

    c.bench_function("Sphere", |b| {
        b.iter_batched(|| raytracer.clone(), |rt| rt.render(), SmallInput);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
