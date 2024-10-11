use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::hint::black_box;

fn generate_random_bytes(buffer: &mut [u8]) {
    let mut rng = StdRng::seed_from_u64(42);
    rng.fill(buffer);
}

fn criterion_benchmark(c: &mut Criterion) {
    // 64 KiB of data
    let mut input = [1u8; 1 << 16];

    generate_random_bytes(&mut input[1 << 7..]);

    // benchmarks
    let mut group = c.benchmark_group("simd-position");
    group.bench_function("next-newline-std", |b| b.iter(|| unsafe { simd_position::next_newline_std(black_box(&input)) }));
    group.bench_function("next-newline-simd", |b| b.iter(|| unsafe { simd_position::next_newline_simd(black_box(&input)) }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
