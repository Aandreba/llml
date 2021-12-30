use criterion::{criterion_group, criterion_main, Criterion};
use llml::vec::EucVecf3;
use rand::random;

fn dot (c: &mut Criterion) {
    let alpha = EucVecf3::new(random(), random(), random());
    let beta = EucVecf3::new(random(), random(), random());

    c.bench_function("Naive Vec3f Dot", |b| {
        b.iter(|| alpha.x * beta.x + alpha.y * beta.y + alpha.z * beta.z)
    });

    c.bench_function("Optimized Vec3d Dot", |b| {
        b.iter(|| alpha.dot(beta))
    });
}

criterion_group!(benches, dot);
criterion_main!(benches);