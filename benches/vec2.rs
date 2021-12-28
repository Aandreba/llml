use std::{arch::aarch64::{float32x2_t, vmul_f32, vaddv_f32}, fmt::{Display, Debug}};

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use llml::EucVecf2;
use rand::random;

#[derive(Debug, Clone, Copy)]
struct BenchInput {
    pub alpha: EucVecf2,
    pub beta: EucVecf2
}

impl Display for BenchInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}

fn mul_scalar (c: &mut Criterion) {

}

fn dot (c: &mut Criterion) {
    let alpha = EucVecf2::new(random(), random());
    let beta = EucVecf2::new(random(), random());

    c.bench_function("Naive", |b| {
        b.iter(|| alpha.x * beta.x + alpha.y * beta.y)
    });

    c.bench_function("Optimized", |b| {
        b.iter(|| alpha.dot(beta))
    });
}

criterion_group!(benches, dot);
criterion_main!(benches);