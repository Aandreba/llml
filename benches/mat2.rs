use criterion::{criterion_group, criterion_main, Criterion};
use llml::{Matf2};
use rand::random;

fn mul (c: &mut Criterion) {
    let alpha = Matf2::of_values(random(), random(), random(), random());
    let beta = Matf2::of_values(random(), random(), random(), random());

    c.bench_function("Naive Mat2f mul", |b| {
        b.iter(|| Matf2::of_values(
            alpha.x.x * beta.x.x + alpha.x.y * beta.y.x, alpha.x.x * beta.x.y + alpha.x.y * beta.y.y,
            alpha.y.x * beta.x.x + alpha.y.y * beta.y.x, alpha.y.x * beta.x.y + alpha.y.y * beta.y.y
        ))
    });

    c.bench_function("Optimized Mat2f mul", |b| {
        b.iter(|| alpha * beta)
    });
}

criterion_group!(benches, mul);
criterion_main!(benches);