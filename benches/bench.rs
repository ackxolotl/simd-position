use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let data = "1|1|4|XvTC9cQ1BnpH3ue|OE|BARBARBAR|ik1yiPkonwJ|pAxmX3VomS5QzAP1K|SPtt5EcqNaafy|hD|525811111|9613662508411977|39|GC|50000.00|.4211|-10.00|1.00|0|0|OsfGdm8UJFMPvwSA3lASWL8Aq941eu2K9bB406Rzp3Z2L3W29631NhfBugH98eeF5JxGNJimKMtwhM0o2F8lKgEw3dPTOSj1cN1w6gTUUk585rG2QIEo0PGO5QZifJWePMXK7GvXrnFrrjeNgbO89eaQiYI2Z7HiSa7RI6bEOGpBxmZ2OHfPsZrzk023YIYksYQfkwAcWOS2vneHL3aoCRliI0HD7gVSt5QnMKOfKSW3GZdUFnK4IU4YFOsAz9G3GuzFWJlshTTGtHXcm8AfrDJKoHtU6jaj5tX8bcUob2O15SX1ayRbVMpp0GwZSZ48KvbAwL37xt9CLZL4mWNc0vMSg3WI9xpG7PVQ3XOb3nTyV7tZ\n".repeat(10000);
    let input = data.as_bytes();

    // benchmarks
    let mut group = c.benchmark_group("simd-position");
    group.bench_function("next-newline-std", |b| b.iter(|| simd_position::next_newline_std(black_box(&input))));
    group.bench_function("next-newline-simd", |b| b.iter(|| simd_position::next_newline_simd(black_box(&input))));
    group.bench_function("split-std", |b| b.iter(|| simd_position::split_std(black_box(&input))));
    group.bench_function("split-simd", |b| b.iter(|| simd_position::split_simd(black_box(&input))));
    group.bench_function("subsplit-std", |b| b.iter(|| simd_position::subsplit_std(black_box(&input))));
    group.bench_function("subsplit-simd", |b| b.iter(|| simd_position::subsplit_simd(black_box(&input))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
