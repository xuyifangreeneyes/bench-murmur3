use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_murmur3::{mur3_fn_multi_values, mur3_hasher_multi_values, murmur3_multi_values};

fn generate_values() -> Vec<Vec<u8>> {
    let mut values = Vec::new();
    for _ in 0..4 {
        let mut value = Vec::with_capacity(100);
        for _ in 0..value.capacity() {
            value.push(rand::random());
        };
        values.push(value);
    }
    values
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("murmur3_hash_for_multi_values");
    let values = generate_values();
    group.bench_with_input(BenchmarkId::new("mur3_fn_multi_values", "3x100"), black_box(&values), 
        |b, v| b.iter(|| mur3_fn_multi_values(v)));
    group.bench_with_input(BenchmarkId::new("mur3_hasher_multi_values", "3x100"), black_box(&values), 
        |b, v| b.iter(|| mur3_hasher_multi_values(v)));
    group.bench_with_input(BenchmarkId::new("murmur3_multi_values", "3x100"), black_box(&values), 
        |b, v| b.iter(|| murmur3_multi_values(v)));
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);