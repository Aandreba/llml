use std::intrinsics::transmute;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3;
use llml::{mat::{Matf3}, vec::EucVecf3};
use rand::random;

/*fn inverse (c: &mut Criterion) {
    let alpha : Matf3 = random();
    let glam = Mat3::from_cols_array(&[
        alpha.x.x, alpha.x.y, alpha.x.z,
        alpha.y.x, alpha.y.y, alpha.y.z,
        alpha.z.x, alpha.z.y, alpha.z.z
    ]);
    
    c.bench_function("Self Mat3f inverse", |b| {
        b.iter(|| alpha.inv())
    });

    c.bench_function("Glam Mat3f inverse", |b| {
        b.iter(|| glam.inverse())
    });
}*/

fn mul (c: &mut Criterion) {
    let alpha : Matf3 = random();
    let beta : Matf3 = random();
    
    c.bench_function("Naive Mat3f mul", |b| {
        b.iter(|| Matf3::from_values(
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

    unsafe {
        c.bench_function("Glam Mat3f mul", |b| {
            let _a : glam::Mat3 = transmute(alpha);
            let _b : glam::Mat3 = transmute(beta);

            b.iter(|| _a * _b)
        });
    }
}

criterion_group!(benches, mul);
criterion_main!(benches);