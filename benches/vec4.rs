use criterion::{criterion_group, criterion_main, Criterion};
use llml::{EucVecf4};
use rand::random;

fn mul (c: &mut Criterion) {
    let alpha = EucVecf4::new(random(), random(), random(), random());
    let beta = EucVecf4::new(random(), random(), random(), random());

    c.bench_function("Naive Vec4f Mul", |b| {
        b.iter(|| EucVecf4::new(alpha.x * beta.x, alpha.y * beta.y, alpha.z * beta.z, alpha.w * beta.w))
    });

    c.bench_function("Optimized Vec4d Mul", |b| {
        b.iter(|| alpha * beta)
    });
}

criterion_group!(benches, mul);
criterion_main!(benches);