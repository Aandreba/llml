use criterion::{criterion_group, criterion_main, Criterion};
use llml::mat::{Matf2, Matf3};
use rand::random;

fn add (c: &mut Criterion) {
    let alpha : Matf3 = random();
    let beta : Matf3 = random();

    c.bench_function("Naive Mat3f add", |b| {
        b.iter(|| Matf3::of_values(
            alpha.x.x + beta.x.x, alpha.x.y + beta.x.y, alpha.x.z + beta.x.z,
            alpha.y.x + beta.y.x, alpha.y.y + beta.y.y, alpha.y.z + beta.y.z,
            alpha.z.x + beta.z.x, alpha.z.y + beta.z.y, alpha.z.z + beta.z.z
        ))
    });

    c.bench_function("Optimized Mat3f add", |b| {
        b.iter(|| alpha + beta)
    });
}

criterion_group!(benches, add);
criterion_main!(benches);