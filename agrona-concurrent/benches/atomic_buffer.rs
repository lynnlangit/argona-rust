use criterion::{black_box, criterion_group, criterion_main, Criterion};
use agrona_concurrent::AtomicBuffer;
use agrona_core::buffer::MutableBuffer;

fn benchmark_atomic_operations(c: &mut Criterion) {
    let mut buffer = AtomicBuffer::new(4096).unwrap();

    c.bench_function("put_volatile_u32", |b| {
        b.iter(|| {
            buffer.put_volatile_u32(0, black_box(0x12345678)).unwrap();
        })
    });

    c.bench_function("get_volatile_u32", |b| {
        b.iter(|| {
            let result = buffer.get_volatile_u32(0).unwrap();
            black_box(result);
        })
    });

    c.bench_function("put_ordered_u64", |b| {
        b.iter(|| {
            buffer.put_ordered_u64(8, black_box(0x123456789abcdef0)).unwrap();
        })
    });

    c.bench_function("get_volatile_u64", |b| {
        b.iter(|| {
            let result = buffer.get_volatile_u64(8).unwrap();
            black_box(result);
        })
    });

    c.bench_function("get_and_add_u32", |b| {
        b.iter(|| {
            let result = buffer.get_and_add_u32(16, black_box(1)).unwrap();
            black_box(result);
        })
    });

    c.bench_function("compare_and_set_u32", |b| {
        b.iter(|| {
            let current = buffer.get_volatile_u32(20).unwrap();
            let result = buffer.compare_and_set_u32(20, current, current.wrapping_add(1)).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_atomic_operations);
criterion_main!(benches);