use criterion::{criterion_group, criterion_main, Criterion};
use llml::EucVecf2;
use rand::random;

fn mul_scalar (c: &mut Criterion) {
    let alpha = EucVecf2::new(random(), random());
    let beta : f32 = random();

    c.bench_function("Naive MulScalar", |b| {
        b.iter(|| EucVecf2::new(alpha.x * beta, alpha.y * beta))
    });

    c.bench_function("Optimized MulScalar", |b| {
        b.iter(|| alpha * beta)
    });
}

fn dot (c: &mut Criterion) {
    let alpha = EucVecf2::new(random(), random());
    let beta = EucVecf2::new(random(), random());

    c.bench_function("Naive Vecf3 dot", |b| {
        b.iter(|| alpha.x * beta.x + alpha.y * beta.y)
    });

    c.bench_function("Optimized Vecf3 dot", |b| {
        b.iter(|| alpha.dot(beta))
    });
}

criterion_group!(benches, dot);
criterion_main!(benches);