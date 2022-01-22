use criterion::{criterion_group, criterion_main, Criterion};
use llml::{EucVecf3, Matf2, EucVecf2};
use rand::random;

pub fn mul(c: &mut Criterion) {
    let a1 = Matf2::new([
        random(), random(), 
        random(), random(),
    ]);

    let a2 = Matf2::new([
        random(), random(), 
        random(), random(),
    ]);

    let b1 = glam::mat2(
        glam::vec2(random(), random()), 
        glam::vec2(random(), random()),
    );

    let b2 = glam::mat2(
        glam::vec2(random(), random()), 
        glam::vec2(random(), random()),
    );

    c.bench_function("glam", |b| {
        b.iter(|| b1.determinant())
    });

    c.bench_function("llml", |b| {
        b.iter(|| a1.det())
    });
}

criterion_group!(benches, mul);
criterion_main!(benches);