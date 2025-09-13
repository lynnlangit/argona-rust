use criterion::{black_box, criterion_group, criterion_main, Criterion};
use agrona_core::bit_util::*;

fn benchmark_bit_operations(c: &mut Criterion) {
    c.bench_function("is_power_of_two", |b| {
        b.iter(|| {
            let mut result = true;
            for i in 1..1000 {
                result &= is_power_of_two(black_box(i));
            }
            black_box(result);
        })
    });

    c.bench_function("next_power_of_two", |b| {
        b.iter(|| {
            let mut sum = 0u32;
            for i in 1..1000 {
                sum = sum.wrapping_add(next_power_of_two(black_box(i)));
            }
            black_box(sum);
        })
    });

    c.bench_function("align", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for i in 1..1000 {
                sum = sum.wrapping_add(align(black_box(i), 64));
            }
            black_box(sum);
        })
    });

    c.bench_function("fast_hex_digit", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for i in 0..16 {
                sum = sum.wrapping_add(fast_hex_digit(black_box(i)));
            }
            black_box(sum);
        })
    });
}

criterion_group!(benches, benchmark_bit_operations);
criterion_main!(benches);