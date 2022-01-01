use criterion::{criterion_group, criterion_main, Criterion};
use llml::{mat::{Matf3}, vec::EucVecf3};
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

/*fn mul (c: &mut Criterion) {
    let alpha : Matf3 = random();
    let beta : Matf3 = random();
    
    c.bench_function("Naive Mat3f mul", |b| {
        b.iter(|| Matf3::of_values(
            alpha.x.x * beta.x.x + alpha.x.y * beta.y.x + alpha.x.z * beta.z.x,
            alpha.y.x * beta.x.x + alpha.y.y * beta.y.x + alpha.y.z * beta.z.x,
            alpha.z.x * beta.x.x + alpha.z.y * beta.y.x + alpha.z.z * beta.z.x,

            alpha.x.x * beta.x.y + alpha.x.y * beta.y.y + alpha.x.z * beta.z.y,
            alpha.y.x * beta.x.y + alpha.y.y * beta.y.y + alpha.y.z * beta.z.y,
            alpha.z.x * beta.x.y + alpha.z.y * beta.y.y + alpha.z.z * beta.z.y,

            alpha.x.x * beta.x.z + alpha.x.y * beta.y.z + alpha.x.z * beta.z.z,
            alpha.y.x * beta.x.z + alpha.y.y * beta.y.z + alpha.y.z * beta.z.z,
            alpha.z.x * beta.x.z + alpha.z.y * beta.y.z + alpha.z.z * beta.z.z
        ))
    });

    c.bench_function("Optimized Mat3f mul", |b| {
        b.iter(|| alpha * beta)
    });
}*/

criterion_group!(benches, add);
criterion_main!(benches);