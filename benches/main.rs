use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use sse_simd_bench::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("portable", |b| {
        b.iter(|| black_box(quat_to_mat3_portable(black_box(&Q))))
    })
    .bench_function("half-simd", |b| {
        b.iter(|| black_box(quat_to_mat3_half_simd(black_box(&Q))))
    })
    .bench_function("simd", |b| {
        b.iter(|| black_box(quat_to_mat3_simd(black_box(&Q))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
