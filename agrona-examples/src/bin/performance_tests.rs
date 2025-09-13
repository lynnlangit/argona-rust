use agrona_core::buffer::{DirectBuffer, MutableBuffer, UnsafeBuffer};
use agrona_collections::{IntHashMap, IntHashSet, MutableInteger};
use agrona_concurrent::{AtomicBuffer, BusySpinIdleStrategy, BackoffIdleStrategy, IdleStrategy};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

const WARM_UP_ITERATIONS: usize = 100_000;
const TEST_ITERATIONS: usize = 1_000_000;
const SMALL_BUFFER_SIZE: usize = 1024;
const LARGE_BUFFER_SIZE: usize = 1024 * 1024;

fn main() {
    println!("🚀 Rust Agrona Performance Test Suite");
    println!("=====================================");
    println!("Warm-up iterations: {}", WARM_UP_ITERATIONS);
    println!("Test iterations: {}", TEST_ITERATIONS);
    println!();

    // Run all performance tests
    buffer_performance_tests();
    collections_performance_tests();
    atomic_operations_tests();
    concurrent_performance_tests();
    memory_usage_tests();

    println!("✅ All performance tests completed!");
}

fn buffer_performance_tests() {
    println!("🔥 Buffer Performance Tests");
    println!("===========================");

    test_buffer_primitive_operations();
    test_buffer_string_operations();
    test_buffer_ascii_number_operations();
    test_buffer_bulk_operations();
    println!();
}

fn test_buffer_primitive_operations() {
    println!("📊 Primitive Operations ({}M iterations)", TEST_ITERATIONS / 1_000_000);

    let mut buffer = UnsafeBuffer::new(LARGE_BUFFER_SIZE).expect("Failed to create buffer");

    // Warm up
    for i in 0..WARM_UP_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_u64(offset, i as u64).unwrap();
        let _ = buffer.get_u64(offset).unwrap();
    }

    // Test u32 operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 4) % (LARGE_BUFFER_SIZE - 4);
        buffer.put_u32(offset, i as u32).unwrap();
    }
    let put_u32_time = start.elapsed();

    let start = Instant::now();
    let mut sum = 0u32;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 4) % (LARGE_BUFFER_SIZE - 4);
        sum = sum.wrapping_add(buffer.get_u32(offset).unwrap());
    }
    let get_u32_time = start.elapsed();

    // Test u64 operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_u64(offset, i as u64).unwrap();
    }
    let put_u64_time = start.elapsed();

    let start = Instant::now();
    let mut sum64 = 0u64;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        sum64 = sum64.wrapping_add(buffer.get_u64(offset).unwrap());
    }
    let get_u64_time = start.elapsed();

    // Test f64 operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_f64(offset, i as f64 * 3.14159).unwrap();
    }
    let put_f64_time = start.elapsed();

    let start = Instant::now();
    let mut sum_f64 = 0.0f64;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        sum_f64 += buffer.get_f64(offset).unwrap();
    }
    let get_f64_time = start.elapsed();

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
    println!("  put_f64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_f64_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_f64_time.as_secs_f64() / 1_000_000.0);
    println!("  get_f64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_f64_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_f64_time.as_secs_f64() / 1_000_000.0);

    // Prevent optimization
    println!("  Checksums: u32={}, u64={}, f64={:.2}", sum, sum64, sum_f64);
}

fn test_buffer_string_operations() {
    println!("📝 String Operations ({}K iterations)", TEST_ITERATIONS / 1000);

    let mut buffer = UnsafeBuffer::new(LARGE_BUFFER_SIZE).expect("Failed to create buffer");
    let test_strings = [
        "AAPL", "GOOGL", "MSFT", "TSLA", "AMZN", "META", "NVDA", "AMD",
        "High-Frequency Trading System", "Ultra-Low Latency Market Data",
        "Real-time Risk Management", "Algorithmic Trading Platform"
    ];

    // Warm up
    for _ in 0..WARM_UP_ITERATIONS / 100 {
        for (i, &s) in test_strings.iter().enumerate() {
            let offset = i * 64;
            buffer.put_string_ascii(offset, s).unwrap();
            let _ = buffer.get_string_ascii(offset).unwrap();
        }
    }

    // Test put_string_ascii
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let string_idx = i % test_strings.len();
        let offset = (i * 64) % (LARGE_BUFFER_SIZE - 64);
        buffer.put_string_ascii(offset, test_strings[string_idx]).unwrap();
    }
    let put_string_time = start.elapsed();

    // Test get_string_ascii
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 64) % (LARGE_BUFFER_SIZE - 64);
        let _ = buffer.get_string_ascii(offset).unwrap();
    }
    let get_string_time = start.elapsed();

    println!("  put_string_ascii: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_string_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_string_time.as_secs_f64() / 1_000_000.0);
    println!("  get_string_ascii: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_string_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_string_time.as_secs_f64() / 1_000_000.0);
}

fn test_buffer_ascii_number_operations() {
    println!("🔢 ASCII Number Operations ({}K iterations)", TEST_ITERATIONS / 1000);

    let mut buffer = UnsafeBuffer::new(LARGE_BUFFER_SIZE).expect("Failed to create buffer");

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 100 {
        let offset = (i * 16) % (LARGE_BUFFER_SIZE - 16);
        buffer.put_i32_ascii(offset, i as i32).unwrap();
        let _ = buffer.parse_i32_ascii(offset, 10).unwrap_or(0);
    }

    // Test put_i32_ascii
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 16) % (LARGE_BUFFER_SIZE - 16);
        buffer.put_i32_ascii(offset, (i as i32).wrapping_sub(500_000)).unwrap();
    }
    let put_i32_ascii_time = start.elapsed();

    // Test parse_i32_ascii
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 16) % (LARGE_BUFFER_SIZE - 16);
        if let Ok(val) = buffer.parse_i32_ascii(offset, 10) {
            sum = sum.wrapping_add(val as i64);
        }
    }
    let parse_i32_ascii_time = start.elapsed();

    println!("  put_i32_ascii:   {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_i32_ascii_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_i32_ascii_time.as_secs_f64() / 1_000_000.0);
    println!("  parse_i32_ascii: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             parse_i32_ascii_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / parse_i32_ascii_time.as_secs_f64() / 1_000_000.0);
    println!("  Checksum: {}", sum);
}

fn test_buffer_bulk_operations() {
    println!("🚛 Bulk Operations");

    let mut buffer = UnsafeBuffer::new(LARGE_BUFFER_SIZE).expect("Failed to create buffer");
    let test_data = vec![0xABu8; 8192];

    // Test bulk put_bytes
    let start = Instant::now();
    for i in 0..(TEST_ITERATIONS / 100) {
        let offset = (i * test_data.len()) % (LARGE_BUFFER_SIZE - test_data.len());
        buffer.put_bytes(offset, &test_data).unwrap();
    }
    let put_bytes_time = start.elapsed();

    // Test bulk get_bytes
    let mut read_buffer = vec![0u8; test_data.len()];
    let start = Instant::now();
    for i in 0..(TEST_ITERATIONS / 100) {
        let offset = (i * test_data.len()) % (LARGE_BUFFER_SIZE - test_data.len());
        buffer.get_bytes(offset, &mut read_buffer).unwrap();
    }
    let get_bytes_time = start.elapsed();

    let bytes_per_op = test_data.len();
    let put_throughput = (TEST_ITERATIONS / 100) * bytes_per_op;
    let get_throughput = (TEST_ITERATIONS / 100) * bytes_per_op;

    println!("  put_bytes (8KB): {:>8.2} ns/op  ({:>8.2} GB/s)",
             put_bytes_time.as_nanos() as f64 / (TEST_ITERATIONS / 100) as f64,
             put_throughput as f64 / put_bytes_time.as_secs_f64() / 1_000_000_000.0);
    println!("  get_bytes (8KB): {:>8.2} ns/op  ({:>8.2} GB/s)",
             get_bytes_time.as_nanos() as f64 / (TEST_ITERATIONS / 100) as f64,
             get_throughput as f64 / get_bytes_time.as_secs_f64() / 1_000_000_000.0);
}

fn collections_performance_tests() {
    println!("🗂️  Collections Performance Tests");
    println!("=================================");

    test_intmap_vs_hashmap();
    test_intset_vs_hashset();
    test_mutable_integer_performance();
    println!();
}

fn test_intmap_vs_hashmap() {
    println!("🗄️  IntHashMap vs HashMap ({}K operations)", TEST_ITERATIONS / 1000);

    // Test Agrona IntHashMap
    let mut agrona_map = IntHashMap::new();

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 10 {
        agrona_map.insert(i as i32, i as i32 * 2);
    }
    agrona_map.clear();

    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        agrona_map.insert(i as i32, i as i32 * 2);
    }
    let agrona_insert_time = start.elapsed();

    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..TEST_ITERATIONS {
        if let Some(&value) = agrona_map.get(&(i as i32)) {
            sum = sum.wrapping_add(value as i64);
        }
    }
    let agrona_lookup_time = start.elapsed();

    // Test std HashMap
    let mut std_map = HashMap::new();

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 10 {
        std_map.insert(i as i32, i as i32 * 2);
    }
    std_map.clear();

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

    println!("  📈 Speedup - Insert: {:.2}x, Lookup: {:.2}x", insert_speedup, lookup_speedup);
    println!("  Checksums: IntHashMap={}, HashMap={}", sum, std_sum);
}

fn test_intset_vs_hashset() {
    println!("🗃️  IntHashSet vs HashSet ({}K operations)", TEST_ITERATIONS / 1000);

    // Test Agrona IntHashSet
    let mut agrona_set = IntHashSet::new();

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 10 {
        agrona_set.insert(i as i32);
    }
    agrona_set.clear();

    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        agrona_set.insert(i as i32);
    }
    let agrona_insert_time = start.elapsed();

    let start = Instant::now();
    let mut agrona_hits = 0;
    for i in 0..TEST_ITERATIONS {
        if agrona_set.contains(i as i32) {
            agrona_hits += 1;
        }
    }
    let agrona_contains_time = start.elapsed();

    // Test std HashSet
    let mut std_set = HashSet::new();

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 10 {
        std_set.insert(i as i32);
    }
    std_set.clear();

    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        std_set.insert(i as i32);
    }
    let std_insert_time = start.elapsed();

    let start = Instant::now();
    let mut std_hits = 0;
    for i in 0..TEST_ITERATIONS {
        if std_set.contains(&(i as i32)) {
            std_hits += 1;
        }
    }
    let std_contains_time = start.elapsed();

    println!("  IntHashSet insert:   {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_insert_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_insert_time.as_secs_f64() / 1_000_000.0);
    println!("  IntHashSet contains: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             agrona_contains_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / agrona_contains_time.as_secs_f64() / 1_000_000.0);

    println!("  HashSet insert:      {:>8.2} ns/op  ({:>8.2} MOps/s)",
             std_insert_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / std_insert_time.as_secs_f64() / 1_000_000.0);
    println!("  HashSet contains:    {:>8.2} ns/op  ({:>8.2} MOps/s)",
             std_contains_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / std_contains_time.as_secs_f64() / 1_000_000.0);

    let insert_speedup = std_insert_time.as_nanos() as f64 / agrona_insert_time.as_nanos() as f64;
    let contains_speedup = std_contains_time.as_nanos() as f64 / agrona_contains_time.as_nanos() as f64;

    println!("  📈 Speedup - Insert: {:.2}x, Contains: {:.2}x", insert_speedup, contains_speedup);
    println!("  Hits: IntHashSet={}, HashSet={}", agrona_hits, std_hits);
}

fn test_mutable_integer_performance() {
    println!("🔢 MutableInteger Operations ({}M operations)", TEST_ITERATIONS / 1_000_000);

    let mut counter = MutableInteger::new(0);

    // Warm up
    for _ in 0..WARM_UP_ITERATIONS {
        counter.increment();
        counter.decrement();
    }
    counter.set(0);

    let start = Instant::now();
    for _ in 0..TEST_ITERATIONS {
        counter.increment();
    }
    let increment_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..TEST_ITERATIONS {
        counter.get();
    }
    let get_time = start.elapsed();

    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        counter.add_and_get(if i % 2 == 0 { 1 } else { -1 });
    }
    let add_and_get_time = start.elapsed();

    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let expected = counter.get();
        counter.compare_and_set(expected, expected + (i as i32));
    }
    let cas_time = start.elapsed();

    println!("  increment:    {:>8.2} ns/op  ({:>8.2} MOps/s)",
             increment_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / increment_time.as_secs_f64() / 1_000_000.0);
    println!("  get:          {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_time.as_secs_f64() / 1_000_000.0);
    println!("  add_and_get:  {:>8.2} ns/op  ({:>8.2} MOps/s)",
             add_and_get_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / add_and_get_time.as_secs_f64() / 1_000_000.0);
    println!("  compare_and_set: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             cas_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / cas_time.as_secs_f64() / 1_000_000.0);

    println!("  Final value: {}", counter.get());
}

fn atomic_operations_tests() {
    println!("⚛️  Atomic Operations Tests");
    println!("==========================");

    test_atomic_buffer_operations();
    test_idle_strategy_performance();
    println!();
}

fn test_atomic_buffer_operations() {
    println!("🔀 AtomicBuffer Operations ({}K operations)", TEST_ITERATIONS / 1000);

    let mut buffer = AtomicBuffer::new(LARGE_BUFFER_SIZE).expect("Failed to create atomic buffer");

    // Warm up
    for i in 0..WARM_UP_ITERATIONS / 100 {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_volatile_u64(offset, i as u64).unwrap();
        let _ = buffer.get_volatile_u64(offset).unwrap();
    }

    // Test volatile operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_volatile_u64(offset, i as u64).unwrap();
    }
    let put_volatile_time = start.elapsed();

    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        sum = sum.wrapping_add(buffer.get_volatile_u64(offset).unwrap());
    }
    let get_volatile_time = start.elapsed();

    // Test ordered operations
    let start = Instant::now();
    for i in 0..TEST_ITERATIONS {
        let offset = (i * 8) % (LARGE_BUFFER_SIZE - 8);
        buffer.put_ordered_u64(offset, i as u64).unwrap();
    }
    let put_ordered_time = start.elapsed();

    // Test atomic arithmetic
    buffer.put_volatile_u64(0, 0).unwrap();
    let start = Instant::now();
    for _ in 0..(TEST_ITERATIONS / 100) {
        buffer.get_and_add_u64(0, 1).unwrap();
    }
    let get_and_add_time = start.elapsed();

    println!("  put_volatile_u64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_volatile_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_volatile_time.as_secs_f64() / 1_000_000.0);
    println!("  get_volatile_u64: {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_volatile_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / get_volatile_time.as_secs_f64() / 1_000_000.0);
    println!("  put_ordered_u64:  {:>8.2} ns/op  ({:>8.2} MOps/s)",
             put_ordered_time.as_nanos() as f64 / TEST_ITERATIONS as f64,
             TEST_ITERATIONS as f64 / put_ordered_time.as_secs_f64() / 1_000_000.0);
    println!("  get_and_add_u64:  {:>8.2} ns/op  ({:>8.2} MOps/s)",
             get_and_add_time.as_nanos() as f64 / (TEST_ITERATIONS / 100) as f64,
             (TEST_ITERATIONS / 100) as f64 / get_and_add_time.as_secs_f64() / 1_000_000.0);

    println!("  Volatile checksum: {}", sum);
    println!("  Final counter: {}", buffer.get_volatile_u64(0).unwrap());
}

fn test_idle_strategy_performance() {
    println!("😴 Idle Strategy Performance");

    let test_duration = Duration::from_millis(100);

    // Test BusySpinIdleStrategy
    let mut busy_spin = BusySpinIdleStrategy::new();
    let start = Instant::now();
    let mut busy_spin_cycles = 0;
    while start.elapsed() < test_duration {
        busy_spin.idle(0);
        busy_spin_cycles += 1;
    }

    // Test BackoffIdleStrategy
    let mut backoff = BackoffIdleStrategy::default();
    let start = Instant::now();
    let mut backoff_cycles = 0;
    while start.elapsed() < test_duration {
        backoff.idle(0);
        backoff_cycles += 1;
    }

    println!("  BusySpinIdleStrategy: {:>10} cycles/100ms ({:.2} MHz)",
             busy_spin_cycles, busy_spin_cycles as f64 / 100.0 / 1000.0);
    println!("  BackoffIdleStrategy:  {:>10} cycles/100ms ({:.2} MHz)",
             backoff_cycles, backoff_cycles as f64 / 100.0 / 1000.0);
}

fn concurrent_performance_tests() {
    println!("🔄 Concurrent Performance Tests");
    println!("===============================");

    test_concurrent_counter();
    test_producer_consumer_throughput();
    println!();
}

fn test_concurrent_counter() {
    println!("🧮 Concurrent Counter (4 threads, {}K increments/thread)", TEST_ITERATIONS / 4000);

    let buffer = Arc::new(Mutex::new(
        AtomicBuffer::new(64).expect("Failed to create buffer")
    ));
    buffer.lock().unwrap().put_volatile_u64(0, 0).unwrap();

    let num_threads = 4;
    let increments_per_thread = TEST_ITERATIONS / 4;
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            let start = Instant::now();
            for _ in 0..increments_per_thread {
                buffer_clone.lock().unwrap().get_and_add_u64(0, 1).unwrap();
            }
            (thread_id, start.elapsed())
        })
    }).collect();

    let start = Instant::now();
    let mut total_thread_time = Duration::new(0, 0);
    for handle in handles {
        let (thread_id, thread_time) = handle.join().unwrap();
        total_thread_time += thread_time;
        println!("  Thread {}: {:>8.2} ns/increment",
                 thread_id,
                 thread_time.as_nanos() as f64 / increments_per_thread as f64);
    }
    let total_time = start.elapsed();

    let final_count = buffer.lock().unwrap().get_volatile_u64(0).unwrap();
    let expected_count = (num_threads * increments_per_thread) as u64;

    println!("  Total time: {:?}", total_time);
    println!("  Average thread time: {:?}", total_thread_time / num_threads);
    println!("  Operations/second: {:.0}", expected_count as f64 / total_time.as_secs_f64());
    println!("  Final count: {} (expected: {})", final_count, expected_count);
    assert_eq!(final_count, expected_count);
}

fn test_producer_consumer_throughput() {
    println!("🏭 Producer-Consumer Throughput ({}K messages)", TEST_ITERATIONS / 1000);

    let buffer = Arc::new(Mutex::new(
        AtomicBuffer::new(65536).expect("Failed to create buffer")
    ));

    // Ring buffer indices
    let head_idx = 0;
    let tail_idx = 8;
    let data_start = 16;
    let message_size = 32;
    let max_messages = (65536 - data_start) / message_size;

    {
        let mut buf = buffer.lock().unwrap();
        buf.put_volatile_u64(head_idx, 0).unwrap();
        buf.put_volatile_u64(tail_idx, 0).unwrap();
    }

    let producer_buffer = Arc::clone(&buffer);
    let consumer_buffer = Arc::clone(&buffer);
    let messages_to_send = TEST_ITERATIONS;

    let producer_handle = thread::spawn(move || {
        let start = Instant::now();
        let mut messages_sent = 0;

        while messages_sent < messages_to_send {
            loop {
                let mut buf = producer_buffer.lock().unwrap();
                let head = buf.get_volatile_u64(head_idx).unwrap() as usize;
                let tail = buf.get_volatile_u64(tail_idx).unwrap() as usize;
                let next_head = (head + 1) % max_messages;

                if next_head != tail {
                    let offset = data_start + (head * message_size);
                    buf.put_u64(offset, messages_sent as u64).unwrap();
                    buf.put_u64(offset + 8, start.elapsed().as_nanos() as u64).unwrap();
                    buf.put_ordered_u64(head_idx, next_head as u64).unwrap();
                    messages_sent += 1;
                    break;
                } else {
                    drop(buf);
                    thread::yield_now();
                }
            }
        }

        (start.elapsed(), messages_sent)
    });

    let consumer_handle = thread::spawn(move || {
        let start = Instant::now();
        let mut messages_received = 0;
        let mut total_latency_ns = 0u64;

        while messages_received < messages_to_send {
            loop {
                let mut buf = consumer_buffer.lock().unwrap();
                let head = buf.get_volatile_u64(head_idx).unwrap() as usize;
                let tail = buf.get_volatile_u64(tail_idx).unwrap() as usize;

                if tail != head {
                    let offset = data_start + (tail * message_size);
                    let _message_id = buf.get_u64(offset).unwrap();
                    let sent_time_ns = buf.get_u64(offset + 8).unwrap();
                    let current_time_ns = start.elapsed().as_nanos() as u64;

                    if current_time_ns > sent_time_ns {
                        total_latency_ns += current_time_ns - sent_time_ns;
                    }

                    let next_tail = (tail + 1) % max_messages;
                    buf.put_ordered_u64(tail_idx, next_tail as u64).unwrap();
                    messages_received += 1;
                    break;
                } else {
                    drop(buf);
                    thread::yield_now();
                }
            }
        }

        let elapsed = start.elapsed();
        let avg_latency_ns = if messages_received > 0 {
            total_latency_ns / messages_received as u64
        } else { 0 };

        (elapsed, messages_received, avg_latency_ns)
    });

    let (producer_time, messages_sent) = producer_handle.join().unwrap();
    let (consumer_time, messages_received, avg_latency_ns) = consumer_handle.join().unwrap();

    println!("  Producer: {} messages in {:?} ({:.0} msg/s)",
             messages_sent, producer_time,
             messages_sent as f64 / producer_time.as_secs_f64());
    println!("  Consumer: {} messages in {:?} ({:.0} msg/s)",
             messages_received, consumer_time,
             messages_received as f64 / consumer_time.as_secs_f64());
    println!("  Average latency: {} ns ({:.2} μs)", avg_latency_ns, avg_latency_ns as f64 / 1000.0);
}

fn memory_usage_tests() {
    println!("💾 Memory Usage Analysis");
    println!("=======================");

    // Test buffer memory overhead
    let buffer_sizes = [1024, 4096, 16384, 65536, 1024 * 1024];
    for &size in &buffer_sizes {
        let buffer = UnsafeBuffer::new(size).expect("Failed to create buffer");
        println!("  UnsafeBuffer({}): {} bytes capacity",
                 format_bytes(size), buffer.capacity());
    }

    // Test collection memory efficiency
    let mut int_map = IntHashMap::new();
    let mut std_map = HashMap::new();

    for i in 0..10000 {
        int_map.insert(i, i * 2);
        std_map.insert(i, i * 2);
    }

    println!("  IntHashMap(10k): {} entries, {} capacity",
             int_map.len(), int_map.capacity());
    println!("  HashMap(10k): {} entries",
             std_map.len());

    let mut int_set = IntHashSet::new();
    let mut std_set = HashSet::new();

    for i in 0..10000 {
        int_set.insert(i);
        std_set.insert(i);
    }

    println!("  IntHashSet(10k): {} entries, {} capacity",
             int_set.len(), int_set.capacity());
    println!("  HashSet(10k): {} entries",
             std_set.len());
}

fn format_bytes(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{}MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{}KB", bytes / 1024)
    } else {
        format!("{}B", bytes)
    }
}