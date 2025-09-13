use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use agrona_core::buffer::{DirectBuffer, MutableBuffer, UnsafeBuffer};

fn benchmark_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_operations");

    for size in [1024, 4096, 16384, 65536].iter() {
        let mut buffer = UnsafeBuffer::new(*size).unwrap();

        group.bench_with_input(BenchmarkId::new("put_u32", size), size, |b, _| {
            b.iter(|| {
                for i in 0..(*size / 4) {
                    let offset = i * 4;
                    buffer.put_u32(offset, black_box(i as u32)).unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("get_u32", size), size, |b, _| {
            b.iter(|| {
                let mut sum = 0u32;
                for i in 0..(*size / 4) {
                    let offset = i * 4;
                    sum += buffer.get_u32(offset).unwrap();
                }
                black_box(sum);
            })
        });

        group.bench_with_input(BenchmarkId::new("put_u64", size), size, |b, _| {
            b.iter(|| {
                for i in 0..(*size / 8) {
                    let offset = i * 8;
                    buffer.put_u64(offset, black_box(i as u64)).unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("get_u64", size), size, |b, _| {
            b.iter(|| {
                let mut sum = 0u64;
                for i in 0..(*size / 8) {
                    let offset = i * 8;
                    sum += buffer.get_u64(offset).unwrap();
                }
                black_box(sum);
            })
        });
    }

    group.finish();
}

fn benchmark_string_operations(c: &mut Criterion) {
    let mut buffer = UnsafeBuffer::new(4096).unwrap();

    c.bench_function("put_string_ascii", |b| {
        b.iter(|| {
            let test_string = "Hello, HFT World! This is a test string for performance measurement.";
            buffer.put_string_ascii(0, black_box(test_string)).unwrap();
        })
    });

    buffer.put_string_ascii(0, "Hello, HFT World! This is a test string for performance measurement.").unwrap();

    c.bench_function("get_string_ascii", |b| {
        b.iter(|| {
            let result = buffer.get_string_ascii(0).unwrap();
            black_box(result);
        })
    });
}

fn benchmark_ascii_numbers(c: &mut Criterion) {
    let mut buffer = UnsafeBuffer::new(1024).unwrap();

    c.bench_function("put_i32_ascii", |b| {
        b.iter(|| {
            buffer.put_i32_ascii(0, black_box(-1234567890)).unwrap();
        })
    });

    buffer.put_i32_ascii(0, -1234567890).unwrap();

    c.bench_function("parse_i32_ascii", |b| {
        b.iter(|| {
            let result = buffer.parse_i32_ascii(0, 11).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(
    benches,
    benchmark_buffer_operations,
    benchmark_string_operations,
    benchmark_ascii_numbers
);
criterion_main!(benches);