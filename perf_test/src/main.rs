use std::time::Instant;
use std::collections::HashMap;
use agrona_core::buffer::{DirectBuffer, MutableBuffer, UnsafeBuffer};
use agrona_collections::{IntHashMap, IntHashSet};

const TEST_ITERATIONS: usize = 1_000_000;

fn main() {
    println!("🚀 Rust Agrona Performance Test Results");
    println!("=========================================");
    println!("Test iterations: {}", TEST_ITERATIONS);
    println!();

    test_buffer_operations();
    test_collections_performance();

    println!("✅ Performance tests completed!");
}

fn test_buffer_operations() {
    println!("📊 Buffer Performance Tests");
    println!("---------------------------");

    let mut buffer = UnsafeBuffer::new(1024 * 1024).expect("Failed to create buffer");

    // Test u32 operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 4) % (1024 * 1024 - 4);
        buffer.put_u32(offset, i as u32).unwrap();
    }
    let put_u32_time = start.elapsed();

    let start = Instant::now();
    let mut sum = 0u32;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 4) % (1024 * 1024 - 4);
        sum = sum.wrapping_add(buffer.get_u32(offset).unwrap());
    }
    let get_u32_time = start.elapsed();

    // Test u64 operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (1024 * 1024 - 8);
        buffer.put_u64(offset, i as u64).unwrap();
    }
    let put_u64_time = start.elapsed();

    let start = Instant::now();
    let mut sum64 = 0u64;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (1024 * 1024 - 8);
        sum64 = sum64.wrapping_add(buffer.get_u64(offset).unwrap());
    }
    let get_u64_time = start.elapsed();

    // Test bulk operations
    let test_data = vec![0xABu8; 8192];
    let start = Instant::now();
    for i in 0..(TEST_ITERATIONS / 100) {
        let offset = (i * test_data.len()) % (1024 * 1024 - test_data.len());
        buffer.put_bytes(offset, &test_data).unwrap();
    }
    let put_bytes_time = start.elapsed();

    println!("  put_u32: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_u32_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_u32_time.as_secs_f64() / 1_000_000.0);
    println!("  get_u32: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_u32_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_u32_time.as_secs_f64() / 1_000_000.0);
    println!("  put_u64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_u64_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_u64_time.as_secs_f64() / 1_000_000.0);
    println!("  get_u64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_u64_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_u64_time.as_secs_f64() / 1_000_000.0);

    let bytes_per_op = test_data.len();
    let throughput = (TEST_ITERATIONS / 100) * bytes_per_op;
    println!("  put_bytes (8KB): {:>8.2} ns/op  ({:>8.2} GB/s)",
             put_bytes_time.as_nanos() as f64 / (TEST_ITERATIONS / 100) as f64,
             throughput as f64 / put_bytes_time.as_secs_f64() / 1_000_000_000.0);

    println!("  Checksums: u32={}, u64={}", sum, sum64);
    println!();
}

fn test_collections_performance() {
    println!("🗂️  Collections Performance Tests");
    println!("---------------------------------");

    // Test IntHashMap vs HashMap
    println!("Testing {} operations", TEST_ITERATIONS);

    // Agrona IntHashMap
    let mut agrona_map = IntHashMap::new();
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        agrona_map.insert(i as i32, i as i32 * 2);
    }
    let agrona_insert_time = start.elapsed();

    let start = Instant::now();
    let mut agrona_sum = 0i64;
    for i in 0..TEST_ITERATIONS {
        if let Some(&value) = agrona_map.get(i as i32) {
            agrona_sum = agrona_sum.wrapping_add(value as i64);
        }
    }
    let agrona_lookup_time = start.elapsed();

    // Standard HashMap
    let mut std_map = HashMap::new();
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        std_map.insert(i as i32, i as i32 * 2);
    }
    let std_insert_time = start.elapsed();

    let start = Instant::now();
    let mut std_sum = 0i64;
    for i in 0..TEST_ITERATIONS {
        if let Some(&value) = std_map.get(&(i as i32)) {
            std_sum = std_sum.wrapping_add(value as i64);
        }
    }
    let std_lookup_time = start.elapsed();

    // Test IntHashSet vs HashSet
    let mut agrona_set = IntHashSet::new();
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        agrona_set.insert(i as i32);
    }
    let agrona_set_insert_time = start.elapsed();

    let start = Instant::now();
    let mut agrona_set_hits = 0;
    for i in 0..TEST_ITERATIONS {
        if agrona_set.contains(i as i32) {
            agrona_set_hits += 1;
        }
    }
    let agrona_set_contains_time = start.elapsed();

    println!("IntHashMap vs HashMap:");
    println!("  IntHashMap insert: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_insert_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_insert_time.as_secs_f64() / 1_000_000.0);
    println!("  IntHashMap lookup: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_lookup_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_lookup_time.as_secs_f64() / 1_000_000.0);
    println!("  HashMap insert:    {:>8.2} ns/op  ({:>8.2} MOps/s)",
             std_insert_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / std_insert_time.as_secs_f64() / 1_000_000.0);
    println!("  HashMap lookup:    {:>8.2} ns/op  ({:>8.2} MOps/s)",
             std_lookup_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / std_lookup_time.as_secs_f64() / 1_000_000.0);

    let insert_speedup = std_insert_time.as_nanos() as f64 / agrona_insert_time.as_nanos() as f64;
    let lookup_speedup = std_lookup_time.as_nanos() as f64 / agrona_lookup_time.as_nanos() as f64;

    println!("  📈 HashMap Speedup - Insert: {:.2}x, Lookup: {:.2}x", insert_speedup, lookup_speedup);

    println!("IntHashSet:");
    println!("  IntHashSet insert:   {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_set_insert_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_set_insert_time.as_secs_f64() / 1_000_000.0);
    println!("  IntHashSet contains: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_set_contains_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_set_contains_time.as_secs_f64() / 1_000_000.0);

    println!("  Memory usage - IntHashMap: {} entries, HashMap: {} entries",
             agrona_map.len(), std_map.len());
    println!("  Checksums: IntHashMap={}, HashMap={}, IntHashSet hits={}",
             agrona_sum, std_sum, agrona_set_hits);
    println!();
}