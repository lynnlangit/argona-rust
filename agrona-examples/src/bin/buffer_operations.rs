use agrona_core::buffer::{DirectBuffer, MutableBuffer, UnsafeBuffer};
use std::time::Instant;

const BUFFER_SIZE: usize = 1024 * 1024;
const ITERATIONS: usize = 1_000_000;

fn main() {
    println!("Agrona Rust Buffer Operations Example");
    println!("=====================================");

    basic_buffer_operations();
    buffer_performance_test();
    string_operations_example();
    ascii_number_operations();
}

fn basic_buffer_operations() {
    println!("\n1. Basic Buffer Operations");
    println!("--------------------------");

    let mut buffer = UnsafeBuffer::new(1024).expect("Failed to create buffer");

    buffer.put_u32(0, 0x12345678).unwrap();
    buffer.put_i64(8, -1234567890123456789i64).unwrap();
    buffer.put_f64(16, std::f64::consts::PI).unwrap();

    println!("Written u32 at 0: 0x{:x}", buffer.get_u32(0).unwrap());
    println!("Written i64 at 8: {}", buffer.get_i64(8).unwrap());
    println!("Written f64 at 16: {}", buffer.get_f64(16).unwrap());

    let test_bytes = b"Hello, Agrona Rust!";
    buffer.put_bytes(32, test_bytes).unwrap();

    let mut read_bytes = vec![0u8; test_bytes.len()];
    buffer.get_bytes(32, &mut read_bytes).unwrap();

    println!("Written/Read bytes: {}", String::from_utf8_lossy(&read_bytes));
}

fn buffer_performance_test() {
    println!("\n2. Buffer Performance Test");
    println!("-------------------------");

    let mut buffer = UnsafeBuffer::new(BUFFER_SIZE).expect("Failed to create buffer");

    println!("Testing {} iterations on {} byte buffer", ITERATIONS, BUFFER_SIZE);

    let start = Instant::now();
    for i in 0..ITERATIONS {
        let offset = (i * 8) % (BUFFER_SIZE - 8);
        buffer.put_u64(offset, i as u64).unwrap();
    }
    let write_time = start.elapsed();

    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..ITERATIONS {
        let offset = (i * 8) % (BUFFER_SIZE - 8);
        sum += buffer.get_u64(offset).unwrap();
    }
    let read_time = start.elapsed();

    println!("Write time: {:?} ({:.2} ns/op)",
             write_time,
             write_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Read time: {:?} ({:.2} ns/op)",
             read_time,
             read_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Checksum (to prevent optimization): {}", sum);

    let total_bytes = ITERATIONS * 8 * 2;
    let total_time = write_time + read_time;
    let throughput = (total_bytes as f64) / total_time.as_secs_f64() / 1024.0 / 1024.0 / 1024.0;
    println!("Total throughput: {:.2} GB/s", throughput);
}

fn string_operations_example() {
    println!("\n3. String Operations");
    println!("-------------------");

    let mut buffer = UnsafeBuffer::new(512).expect("Failed to create buffer");

    let test_strings = [
        "Hello, World!",
        "High-Frequency Trading",
        "Zero-Copy Buffer Operations",
        "Rust Performance Rocks",
    ];

    let mut offset = 0;
    for (i, &s) in test_strings.iter().enumerate() {
        println!("Writing string {}: '{}'", i, s);

        let bytes_written = buffer.put_string_ascii(offset, s).unwrap();
        println!("Bytes written: {}", bytes_written);

        let read_string = buffer.get_string_ascii(offset).unwrap();
        println!("Read back: '{}'", read_string);

        assert_eq!(s, read_string);
        offset += bytes_written;
        println!();
    }
}

fn ascii_number_operations() {
    println!("\n4. ASCII Number Operations");
    println!("-------------------------");

    let mut buffer = UnsafeBuffer::new(256).expect("Failed to create buffer");

    let test_numbers = [42, -123, 1234567890, -987654321];

    let mut offset = 0;
    for &num in &test_numbers {
        println!("Writing number: {}", num);

        let bytes_written = buffer.put_i32_ascii(offset, num).unwrap();
        println!("Bytes written: {}", bytes_written);

        let read_number = buffer.parse_i32_ascii(offset, bytes_written).unwrap();
        println!("Read back: {}", read_number);

        assert_eq!(num, read_number);
        offset += bytes_written + 1;
        buffer.put_u8(offset - 1, b' ').unwrap();
    }

    let mut ascii_repr = vec![0u8; offset];
    buffer.get_bytes(0, &mut ascii_repr).unwrap();
    println!("ASCII representation: '{}'", String::from_utf8_lossy(&ascii_repr));

    let natural_numbers = [0u32, 1, 42, 123456789, i32::MAX as u32];

    println!("\nNatural number operations:");
    offset = 0;
    for &num in &natural_numbers {
        println!("Writing natural number: {}", num);

        let bytes_written = buffer.put_natural_i32_ascii(offset, num as i32).unwrap();
        let read_number = buffer.parse_natural_i32_ascii(offset, bytes_written).unwrap();

        println!("Read back: {}", read_number);
        assert_eq!(num as i32, read_number);
        offset += bytes_written + 1;
        buffer.put_u8(offset - 1, b',').unwrap();
    }

    let mut final_repr = vec![0u8; offset];
    buffer.get_bytes(0, &mut final_repr).unwrap();
    println!("Final representation: '{}'", String::from_utf8_lossy(&final_repr));
}