use criterion::{criterion_group, criterion_main, Criterion};
use llml::EucVecf3;
use rand::random;

pub fn sum(c: &mut Criterion) {
    let a1 = EucVecf3::new(random(), random(), random());
    let a2 = EucVecf3::new(random(), random(), random());

    let b1 = glam::vec3a(a1.x(), a1.y(), a1.z());
    let b2 = glam::vec3a(a2.x(), a2.y(), a2.z());

    c.bench_function("glam", |b| {
        b.iter(|| b1 + b2)
    });

    c.bench_function("llml", |b| {
        b.iter(|| a1 + a2)
    });
}

criterion_group!(benches, sum);
criterion_main!(benches);