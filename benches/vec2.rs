use criterion::{criterion_group, criterion_main, Criterion};
use llml::vec::EucVecf2;
use rand::random;

fn dot (c: &mut Criterion) {
    let alpha = EucVecf2::new(random(), random());
    let beta = EucVecf2::new(random(), random());

    c.bench_function("Naive Vecf2 dot", |b| {
        b.iter(|| alpha.x * beta.x + alpha.y * beta.y)
    });

    c.bench_function("Optimized Vecf2 dot", |b| {
        b.iter(|| alpha.dot(beta))
    });
}

criterion_group!(benches, dot);
criterion_main!(benches);