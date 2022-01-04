#![feature(portable_simd)]
use std::{arch::aarch64::{vaddv_f32, vmul_f32, vmulq_f32, vaddvq_f32}, intrinsics::transmute, simd::Simd};

use criterion::{criterion_group, criterion_main, Criterion};
use llml::vec::{EucVecf2, EucVecf4};
use rand::random;

fn simd (c: &mut Criterion) {
    let alpha = EucVecf4::new(random(), random(), random(), random());
    let beta = EucVecf4::new(random(), random(), random(), random());

    unsafe {
        c.bench_function("ARM intrinsic", |b| {
            b.iter(|| vaddvq_f32(vmulq_f32(transmute(alpha), transmute(beta))));
        });
        
        c.bench_function("SIMD abstract", |b| {
            b.iter(|| (transmute::<EucVecf4, Simd<f32,4>>(alpha) * transmute::<EucVecf4, Simd<f32,4>>(beta)).horizontal_sum())
        });
    }
}

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

criterion_group!(benches, simd);
criterion_main!(benches);