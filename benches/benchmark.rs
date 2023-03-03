use criterion::{criterion_group, criterion_main, Criterion};
use raytracing_in_one_weekend::*;

fn criterion_benchmark(c: &mut Criterion) {
    let camera = Camera::default();
    let raytracer = Raytracer::new(camera, 160, 90, 10, 10);

    c.bench_function("Sphere", |b| {
        b.iter(|| {
            raytracer.render();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
