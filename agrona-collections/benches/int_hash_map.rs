use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use agrona_collections::IntHashMap;
use std::collections::HashMap;

fn benchmark_int_hash_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("int_hash_map");

    for size in [1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("agrona_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut map = IntHashMap::new();
                for i in 0..size {
                    map.insert(black_box(i as i32), black_box(i as i32 * 2));
                }
                black_box(map);
            })
        });

        group.bench_with_input(BenchmarkId::new("std_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut map = HashMap::new();
                for i in 0..size {
                    map.insert(black_box(i as i32), black_box(i as i32 * 2));
                }
                black_box(map);
            })
        });

        // Lookup benchmarks
        let mut agrona_map = IntHashMap::new();
        let mut std_map = HashMap::new();
        for i in 0..*size {
            agrona_map.insert(i as i32, i as i32 * 2);
            std_map.insert(i as i32, i as i32 * 2);
        }

        group.bench_with_input(BenchmarkId::new("agrona_lookup", size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0;
                for i in 0..size {
                    if let Some(&value) = agrona_map.get(&(i as i32)) {
                        sum += value;
                    }
                }
                black_box(sum);
            })
        });

        group.bench_with_input(BenchmarkId::new("std_lookup", size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0;
                for i in 0..size {
                    if let Some(&value) = std_map.get(&(i as i32)) {
                        sum += value;
                    }
                }
                black_box(sum);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_int_hash_map);
criterion_main!(benches);